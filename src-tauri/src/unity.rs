use std::sync::OnceLock;

use dashmap::{self, DashMap};

use uuid::Uuid;

use self::dto::StoreUnityAsset;
mod commands;
mod dto;
mod error;
mod loader;
static OPEND_UNITY_ASSET: OnceLock<dashmap::DashMap<Uuid, StoreUnityAsset>> = OnceLock::new();

type UnityResult<T> = Result<T, error::Error>;

fn get_opend_unity_asset() -> &'static DashMap<Uuid, StoreUnityAsset> {
    OPEND_UNITY_ASSET.get_or_init(DashMap::new)
}

pub use commands::{load_unity_asset, preview_object};
