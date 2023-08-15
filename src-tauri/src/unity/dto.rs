use std::{fs::write, path::Path, sync::Arc};

use image::RgbaImage;

use unity_rs::{ClassID, Env};

use super::{loaded_object::LoadedObject, UnityResult};

#[derive(Debug, Clone, serde::Serialize)]
pub struct UnityAsset {
    pub id: String,
    pub name: String,
    pub location: String,
    pub assets: Vec<UnityObject>,
}

impl UnityAsset {
    pub fn from_store(id: &uuid::Uuid, store: &StoreUnityBundle) -> Self {
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
                    name: data.name.clone(),
                    ty: format!("{:?}", ty),
                    meta: data.meta.clone(),
                    icon: data.icon,
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct UnityObject {
    pub id: usize,
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub meta: Vec<String>,
    pub icon: &'static str,
}

pub trait Loadable: Send + Sync {
    fn icon() -> &'static str {
        "mid-package-variant-closed"
    }

    fn name(&self) -> String;

    fn meta(&self) -> Vec<String>;

    fn load_preview(&self) -> UnityResult<LoadedPayload>;
}

pub enum LoadedPayload {
    Image(RgbaImage),
    Text(String),
    Raw(Vec<u8>),
}

impl LoadedPayload {
    pub fn save(&self, path: impl AsRef<Path>) -> UnityResult<()> {
        match self {
            LoadedPayload::Image(img) => img.save_with_format(path, image::ImageFormat::Png)?,
            LoadedPayload::Text(s) => write(path, s.as_bytes())?,
            LoadedPayload::Raw(s) => write(path, s.as_slice())?,
        }
        Ok(())
    }

    pub fn get_file_extension(&self) -> &'static str {
        match self {
            LoadedPayload::Image(_) => "png",
            LoadedPayload::Text(_) => "txt",
            LoadedPayload::Raw(_) => "binary",
        }
    }

    pub fn get_file_extension_name(&self) -> &'static str {
        match self {
            LoadedPayload::Image(_) => "PNG Image File",
            LoadedPayload::Text(_) => "Text File",
            LoadedPayload::Raw(_) => "Raw Binary File",
        }
    }

    pub fn get_file_name(&self, name: &str) -> String {
        format!("{name}.{}", self.get_file_extension())
    }
}

pub struct StoreUnityBundle {
    pub origin: Env,
    pub objects: Vec<(ClassID, Arc<LoadedObject>)>,
    pub location: String,
    pub name: String,
}
