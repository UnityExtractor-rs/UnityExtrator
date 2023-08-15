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
    pub fn from_store(id: &uuid::Uuid, store: &StoreUnityBoundle) -> Self {
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

    fn load_preview(&self) -> UnityResult<LoadedPyaload>;
}

pub enum LoadedPyaload {
    Image(RgbaImage),
    Text(String),
    Raw(Vec<u8>),
}

impl LoadedPyaload {
    pub fn save(&self, path: impl AsRef<Path>) -> UnityResult<()> {
        match self {
            LoadedPyaload::Image(img) => img.save_with_format(path, image::ImageFormat::Png)?,
            LoadedPyaload::Text(s) => write(path, s.as_bytes())?,
            LoadedPyaload::Raw(s) => write(path, s.as_slice())?,
        }
        Ok(())
    }

    pub fn get_file_extension(&self) -> &'static str {
        match self {
            LoadedPyaload::Image(_) => "png",
            LoadedPyaload::Text(_) => "txt",
            LoadedPyaload::Raw(_) => "binary",
        }
    }

    pub fn get_file_extension_name(&self) -> &'static str {
        match self {
            LoadedPyaload::Image(_) => "PNG Image File",
            LoadedPyaload::Text(_) => "Text File",
            LoadedPyaload::Raw(_) => "Raw Binary File",
        }
    }

    pub fn get_file_name(&self, name: &str) -> String {
        format!("{name}.{}", self.get_file_extension())
    }
}

pub struct StoreUnityBoundle {
    pub origin: Env,
    pub objects: Vec<(ClassID, Arc<LoadedObject>)>,
    pub location: String,
    pub name: String,
}
