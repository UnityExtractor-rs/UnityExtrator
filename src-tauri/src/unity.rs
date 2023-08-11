use std::cell::OnceCell;
use std::fs::read;
use std::io;
use std::io::Cursor;
use std::sync::OnceLock;
use base64::Engine;
use bytes::BytesMut;
use image::{ImageError, ImageOutputFormat, save_buffer_with_format};
use image::codecs::png::PngEncoder;
use serde::{Deserialize, Serialize};
use tauri::command;
use unity_rs;
use unity_rs::{ClassID, Env, Sprite, UnityError, UnityResult};
use unity_rs::classes::Texture2D;


static ENV:OnceLock<Env> = OnceLock::new();

#[derive(Debug,Serialize,Deserialize)]
pub struct ThisError {
    msg: String,
}

impl From<UnityError> for ThisError {
    fn from(value: UnityError) -> Self {
        Self{
            msg:format!("UnityError:{value}")
        }
    }
}

impl From<io::Error> for ThisError {
    fn from(value: io::Error) -> Self {
        Self{
            msg:format!("IOError:{value}")
        }
    }
}

impl From<ImageError> for ThisError {
    fn from(value: ImageError) -> Self {
        Self{
            msg:format!("ImageError:{value}")
        }
    }
}


#[command]
pub fn extractor_unity_img(filename:String)->Result<String,ThisError>{
    let mut env = Env::new();
    let payload = read(filename).expect("error");
    env.load_from_slice(&payload)?;

    println!("{:?}",env.objects().map(|obj|obj.class()).collect::<Vec<_>>());
    if let Some(obj) = env.objects().find(|obj|obj.class() == ClassID::Texture2D){
        println!("now load obj");
        let s:Texture2D = obj.read().expect("cannot unpack");
        println!("now load done {:?}",s.format);
        let img = s.decode_image()?;
        let mut buffer = Vec::new();
        let png = PngEncoder::new(&mut buffer);
        img.write_with_encoder(png)?;

        let encoded = base64::engine::general_purpose::STANDARD.encode(buffer);
    Ok(format!("data:image/png;base64,{}",encoded))
    }else {
        Err(ThisError{msg:"No Texture Found".into()})
    }

}