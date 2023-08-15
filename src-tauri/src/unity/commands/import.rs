use std::path::PathBuf;

use tauri::{command, AppHandle};
use uuid::Uuid;

use crate::{
    loading_done,
    unity::{
        dto::{StoreUnityBundle, UnityAsset},
        error::Error,
        get_opened_unity_asset, UnityResult,
    },
};

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
#[command]
pub fn sync_loaded_asset() -> Vec<UnityAsset> {
    get_opened_unity_asset()
        .iter()
        .map(|item| UnityAsset::from_store(item.key(), item.value()))
        .collect()
}
