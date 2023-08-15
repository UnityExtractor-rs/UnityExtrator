use std::path::PathBuf;

use tauri::{command, AppHandle, Manager};
use uuid::Uuid;

use crate::{
    loading_done,
    unity::{
        dto::{StoreUnityBundle, UnityAsset},
        error::Error,
        get_opened_unity_asset, UnityResult,
    },
};

const LOAD_ASSET_DONE: &str = "unity-bundle-load-done";

#[command(async)]
pub async fn load_unity_asset(app: AppHandle, path: String) -> UnityResult<UnityAsset> {
    let key = Uuid::new_v5(&Uuid::NAMESPACE_OID, path.as_bytes());

    let asset = if let Some(loaded) = get_opened_unity_asset().get(&key) {
        UnityAsset::from_store(&key, &loaded)
    } else {
        let filename = PathBuf::from(path.clone());
        let filename = filename
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or_else(|| Error::UnknownPath(path.clone()))?;
        //load
        let store = StoreUnityBundle::load(&path, filename)?;
        let asset = UnityAsset::from_store(&key, &store);
        get_opened_unity_asset().insert(key, store);
        asset
    };
    loading_done(app)?;
    Ok(asset)
}

pub async fn dropped_unity_asset(app: AppHandle, paths: Vec<PathBuf>) -> UnityResult<()> {
    app.emit_all("loading", true)?;
    let mut result = Vec::new();
    for path in paths.into_iter().filter(|path| path.is_file()) {
        let Some(path) = path.as_os_str().to_str() else{
            continue;
        };
        if let Ok(ua) = load_unity_asset(app.clone(), path.to_string()).await {
            result.push(ua)
        }
    }
    app.emit_to("main", LOAD_ASSET_DONE, result)?;
    Ok(())
}

#[command]
pub fn sync_loaded_asset() -> Vec<UnityAsset> {
    get_opened_unity_asset()
        .iter()
        .map(|item| UnityAsset::from_store(item.key(), item.value()))
        .collect()
}
