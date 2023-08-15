use std::sync::Arc;

use dashmap::DashMap;
use uuid::Uuid;

use super::{dto::StoreUnityBundle, error::Error, loaded_object::LoadedObject, UnityResult};

pub(super) fn get_object(
    map: &DashMap<Uuid, StoreUnityBundle>,
    asset_id: Uuid,
    object_id: usize,
) -> UnityResult<Arc<LoadedObject>> {
    let asset = map
        .get(&asset_id)
        .ok_or_else(|| Error::AssetNotLoaded(asset_id))?;
    let (_class, object) = asset
        .objects
        .get(object_id)
        .ok_or_else(|| Error::ObjectNotExist(asset.name.clone(), object_id))?;

    Ok(Arc::clone(object))
}

