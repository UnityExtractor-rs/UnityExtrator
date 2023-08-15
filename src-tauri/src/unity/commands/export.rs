use std::path::PathBuf;

use tauri::{command, AppHandle};
use uuid::Uuid;

use crate::{
    loading_done,
    unity::{error::Error, get_opened_unity_asset, utils::get_object, UnityResult},
};

#[command(async)]
pub async fn export_bundle(app: AppHandle, dir: String, asset_id: Uuid) -> UnityResult<()> {
    let path = PathBuf::from(dir);
    if !path.exists() {
        std::fs::create_dir_all(&path)?;
    }

    let asset = get_opened_unity_asset()
        .get(&asset_id)
        .ok_or_else(|| Error::AssetNotLoaded(asset_id))?;

    for (class, obj) in &asset.objects {
        let payload = &obj.payload;
        let filename = payload.get_file_name(&obj.name);
        let filename = path.join(format!("[{class:?}]{filename}"));

        payload.save(filename)?;
    }

    loading_done(app)?;
    Ok(())
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct FileExtension {
    name: &'static str,
    wildcard: &'static str,
}
#[command]
pub fn export_file_type(asset_id: Uuid, object_id: usize) -> UnityResult<FileExtension> {
    let object = get_object(get_opened_unity_asset(), asset_id, object_id)?;
    Ok(FileExtension {
        name: object.payload.get_file_extension_name(),
        wildcard: object.payload.get_file_extension(),
    })
}
#[command(async)]
pub async fn export_object(
    app: AppHandle,
    filename: String,
    asset_id: Uuid,
    object_id: usize,
) -> UnityResult<()> {
    let object = get_object(get_opened_unity_asset(), asset_id, object_id)?;
    object.payload.save(filename)?;
    loading_done(app)?;
    Ok(())
}
