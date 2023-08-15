use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use image::{codecs::png::PngEncoder, RgbaImage};
use tauri::{command, AppHandle};
use uuid::Uuid;

use crate::{
    preview::{preview_image, preview_text, ImagePreviewPayload, TextPreviewPayload},
    unity::{
        dto::LoadedPyaload, error::Error, get_opend_unity_asset, loaded_object::LoadedObject,
        utils::get_object, UnityResult,
    },
};

#[command(async)]
pub async fn preview_object(app: AppHandle, asset_id: Uuid, object_id: usize) -> UnityResult<()> {
    // load
    let object = get_object(get_opend_unity_asset(), asset_id, object_id)?;

    // encoding image
    match &object.payload {
        LoadedPyaload::Image(img) => {
            let preview = handle_image_preview(img, &object)?;
            preview_image(app, preview).await?;
        }
        LoadedPyaload::Text(text) => {
            let preview = handle_text_preview(&text, &object)?;
            preview_text(app, preview).await?;
        }
        LoadedPyaload::Raw(_) => {
            eprintln!("Raw Preview not support");
            Err(Error::RawNotPreviewAble)?;
        }
    }

    Ok(())
}

fn handle_image_preview(
    img: &RgbaImage,
    object: &LoadedObject,
) -> UnityResult<ImagePreviewPayload> {
    let mut buffer = Vec::new();
    let png = PngEncoder::new(&mut buffer);
    img.write_with_encoder(png)?;

    let mut base64_encoded = STANDARD.encode(buffer);
    base64_encoded.insert_str(0, "data:image/png;base64,");
    // preview
    let payload = ImagePreviewPayload {
        name: object.name.clone(),
        description: Some(object.meta.join(", ")),
        width: Some(img.width() as _),
        height: Some(img.height() as _),
        image_url: base64_encoded,
        fromat: "Png".to_string(),
    };
    Ok(payload)
}

fn handle_text_preview(payload: &str, obj: &LoadedObject) -> UnityResult<TextPreviewPayload> {
    Ok(TextPreviewPayload {
        payload: payload.to_string(),
        name: obj.name.clone(),
        description: Some(obj.meta.join(", ")),
    })
}
