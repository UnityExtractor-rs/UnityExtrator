use std::path::PathBuf;

use crate::preview::PreviewPayload;
use crate::unity::error::Error;
use crate::unity::get_opend_unity_asset;
use crate::{preview::preview_image, unity::dto::StoreUnityAsset};

use super::{dto::UnityAsset, UnityResult};

use base64::engine::general_purpose::STANDARD;
use base64::Engine;

use image::codecs::png::PngEncoder;
use tauri::{command, AppHandle, Manager};
use uuid::Uuid;

#[command]
pub fn load_unity_asset(app:AppHandle,path: String) -> UnityResult<UnityAsset> {
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
        let store = StoreUnityAsset::load(&path, filename)?;
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
    let (class, object) = asset
        .objects
        .get(object_id)
        .ok_or_else(|| Error::ObjectNotExist(asset.name.clone(), object_id))?;

    // encoding image
    let img = object.load_preview()?;
    let mut buffer = Vec::new();
    let png = PngEncoder::new(&mut buffer);
    img.write_with_encoder(png)?;

    let mut base64_encoded = STANDARD.encode(buffer);
    base64_encoded.insert_str(0, "data:image/png;base64,");
    // preview
    let payload = PreviewPayload {
        obj_url: base64_encoded,
        name: object.get_name(),
        description: Some(object.fetch_meta().join(" ")),
        ty: Some(format!("{:?}", class)),
        width: Some(img.width() as _),
        height: Some(img.height() as _),
    };

    preview_image(app, payload).await?;
    Ok(())
}
