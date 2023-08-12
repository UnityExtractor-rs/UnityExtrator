use std::io::Read;

use dashmap::mapref::one::{self, Ref};
use image::RgbaImage;
use unity_rs::{
    classes::{TextAsset, Texture2D},
    ClassID, Env, Sprite,
};

use super::{
    dto::{Loadable, Preview, StoreUnityAsset},
    UnityResult,
};

impl Loadable for Texture2D {
    fn get_icon(&self) -> &'static str {
        "mdi-image"
    }
    fn get_name(&self) -> String {
        self.name.to_owned()
    }

    fn fetch_meta(&self) -> Vec<String> {
        vec![
            format!("{:?}", self.format),
            format!("({} * {})", self.width, self.height),
        ]
    }

    fn load_preview(&self) -> super::UnityResult<Preview<'_, i64, RgbaImage>> {
        let img = self.decode_image()?;
        Ok(Preview::Cache(img))
    }
}

impl Loadable for TextAsset {
    fn get_icon(&self) -> &'static str {
        "mdi-text-box"
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn fetch_meta(&self) -> Vec<String> {
        vec![format!("script len:{}", self.script.len())]
    }

    fn load_preview(&self) -> UnityResult<Preview<'_, i64, RgbaImage>> {
        Err(super::error::Error::ObjectNotSupportPreview(
            ClassID::TextAsset,
        ))
    }
}

// impl Loadable for Sprite<'_> {
//     fn get_name(&self) -> String {
//         self.name.clone()
//     }

//     fn fetch_meta(&self) -> Vec<String> {
//         vec![format!(
//             "x: {:.02}, y: {:.02}, width: {:.02}, height: {:.02}",
//             self.rect.x, self.rect.y, self.rect.w, self.rect.h
//         )]
//     }

//     fn load_preview(&self) -> super::UnityResult<Preview<'_, i64, RgbaImage>> {
//         let img = self.decode_image()?;
//         Ok(Preview::Owned(img))
//     }
// }

impl StoreUnityAsset {
    pub fn load(path: &str, name: &str) -> UnityResult<Self> {
        let mut env = Env::new();
        let mut file = std::fs::OpenOptions::new().read(true).open(path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;

        env.load_from_slice(&buf)?;

        let mut objects = Vec::new();
        for obj in env.objects().filter_map(|obj| match obj.class() {
            c @ ClassID::Texture2D => Some(
                obj.read::<Texture2D>()
                    .map(|v| (c, Box::new(v) as Box<dyn Loadable>)),
            ),
            c @ ClassID::TextAsset => Some(
                obj.read::<TextAsset>()
                    .map(|v| (c, Box::new(v) as Box<dyn Loadable>)),
            ),
            _ => None,
        }) {
            let obj = obj?;
            objects.push(obj)
        }

        Ok(Self {
            objects,
            location: path.to_string(),
            name: name.to_string(),
        })
    }
}
