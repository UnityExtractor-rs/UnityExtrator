use std::sync::OnceLock;

use dashmap::{self, DashMap};

use uuid::Uuid;

use self::dto::StoreUnityBundle;
mod commands;
mod dto;
mod error;
mod loaded_object;
mod loader;
mod utils;
static OPENED_UNITY_ASSET: OnceLock<DashMap<Uuid, StoreUnityBundle>> = OnceLock::new();

type UnityResult<T> = Result<T, error::Error>;

fn get_opened_unity_asset() -> &'static DashMap<Uuid, StoreUnityBundle> {
    OPENED_UNITY_ASSET.get_or_init(DashMap::new)
}

pub use commands::{
    export_bundle, export_file_type, export_object, load_unity_asset, preview_object,
    sync_loaded_asset,dropped_unity_asset
};
