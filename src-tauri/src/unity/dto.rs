use std::{any::Any, collections::HashMap, path::PathBuf, ops::Deref, hash::Hash};

use dashmap::mapref::one::Ref;
use image::{error, RgbaImage};
use serde::Serializer;
use unity_rs::ClassID;

use super::UnityResult;

#[derive(Debug, Clone, serde::Serialize)]
pub struct UnityAsset {
    pub id: String,
    pub name: String,
    pub location: String,
    pub assets: Vec<UnityObject>,
}

impl UnityAsset {
    pub fn from_store(id: &uuid::Uuid, store: &StoreUnityAsset) -> Self {
        Self {
            id: id.to_string(),
            name: store.name.clone(),
            location: store.location.clone(),
            assets: store
                .objects
                .iter()
                .enumerate()
                .map(|(id, (ty, data))| UnityObject {
                    id,
                    name: data.get_name(),
                    ty: format!("{:?}", ty),
                    meta: data.fetch_meta(),
                    icon: data.get_icon(),
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct UnityObject {
    pub id: usize,
    pub name: String,
    #[serde(rename= "type")]
    pub ty: String,
    pub meta: Vec<String>,
    pub icon: &'static str,
}

pub trait Loadable: Any + Send + Sync + 'static {
    fn get_icon(&self) -> &'static str {
        "mid-package-variant-closed"
    }

    fn get_name(&self) -> String;

    fn fetch_meta(&self) -> Vec<String>;

    fn load_preview(&self) -> UnityResult<Preview<'_, i64, RgbaImage>>;
}

pub enum Preview<'a, K, T> {
    Cache(Ref<'a, K, T>),
    Owned(T),
}

impl<'a, K:Eq + Hash, T> Deref for Preview<'a, K, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Preview::Cache(ref_data) => ref_data.value(),
            Preview::Owned(data) => data,
        }
    }
}

pub struct StoreUnityAsset {
    pub objects: Vec<(ClassID, Box<dyn Loadable>)>,
    pub location: String,
    pub name: String,
}
