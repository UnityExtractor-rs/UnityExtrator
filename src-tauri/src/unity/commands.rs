use std::path::PathBuf;

use crate::preview::{preview_text, ImagePreviewPayload, TextPreviewPayload};
use crate::unity::error::{self, Error};
use crate::unity::get_opend_unity_asset;
use crate::{preview::preview_image, unity::dto::StoreUnityBoundle};

use super::loaded_object::LoadedObject;
use super::{dto::UnityAsset, UnityResult};

use base64::engine::general_purpose::STANDARD;
use base64::Engine;

use image::codecs::png::PngEncoder;
use image::RgbaImage;
use tauri::{command, AppHandle, Manager};

use uuid::Uuid;

#[command]
pub fn load_unity_asset(app: AppHandle, path: String) -> UnityResult<UnityAsset> {
    let key = Uuid::new_v5(&Uuid::NAMESPACE_OID, path.as_bytes());

    let asset = if let Some(loaded) = get_opend_unity_asset().get(&key) {
        UnityAsset::from_store(&key, &loaded)
    } else {
        let filename = PathBuf::from(path.clone());
        let filename = filename
            .file_name()
            .map(|s| s.to_str())
            .flatten()
            .ok_or_else(|| Error::UnknownPath(path.clone()))?;
        //load
        let store = StoreUnityBoundle::load(&path, filename)?;
        let asset = UnityAsset::from_store(&key, &store);
        get_opend_unity_asset().insert(key, store);
        asset
    };
    app.emit_all("loading", false)?;
    Ok(asset)
}

#[command(async)]
pub async fn preview_object(app: AppHandle, asset_id: Uuid, object_id: usize) -> UnityResult<()> {
    // load
    let asset = get_opend_unity_asset()
        .get(&asset_id)
        .ok_or_else(|| Error::AssetNotLoaded(asset_id))?;
    let (_class, object) = asset
        .objects
        .get(object_id)
        .ok_or_else(|| Error::ObjectNotExist(asset.name.clone(), object_id))?;

    // encoding image
    match &object.payload {
        super::dto::LoadedPyaload::Image(img) => {
            let preview = handle_image_preview(img, object)?;
            preview_image(app, preview).await?;
        }
        super::dto::LoadedPyaload::Text(text) => {
            let preview = handle_text_preview(&text, object)?;
            preview_text(app, preview).await?;
        }
        super::dto::LoadedPyaload::Raw(_) => {
            eprintln!("Raw Preview not support");
            Err(error::Error::RawNotPreviewAble)?;
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
