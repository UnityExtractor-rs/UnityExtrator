use std::sync::OnceLock;

use dashmap::{self, DashMap};

use uuid::Uuid;

use self::dto::StoreUnityBoundle;
mod commands;
mod dto;
mod error;
mod loaded_object;
mod loader;
mod utils;
static OPEND_UNITY_ASSET: OnceLock<dashmap::DashMap<Uuid, StoreUnityBoundle>> = OnceLock::new();

type UnityResult<T> = Result<T, error::Error>;

fn get_opend_unity_asset() -> &'static DashMap<Uuid, StoreUnityBoundle> {
    OPEND_UNITY_ASSET.get_or_init(DashMap::new)
}

pub use commands::{
    export_bundle, export_file_type, export_object, load_unity_asset, preview_object,
};
