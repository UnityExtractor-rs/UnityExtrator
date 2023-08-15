use std::io;

use image::ImageError;
use unity_rs::UnityError;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("UnityError: {0}")]
    Unity(#[from] UnityError),
    #[error("IoError: {0}")]
    Io(#[from] io::Error),
    #[error("ImageError: {0}")]
    Image(#[from] ImageError),
    #[error("UnknownPath: {0:?}")]
    UnknownPath(String),
    #[error("Tauri: {0}")]
    Tauri(#[from] tauri::Error),
    #[error("Asset[{0:?}] Not Loaded, Please load first")]
    AssetNotLoaded(Uuid),
    #[error("Asset[{0}] Not Have Object[{1}]")]
    ObjectNotExist(String, usize),

    #[error("Row TextAsset cannot been preview")]
    RawNotPreviewAble,
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let data = self.to_string();
        data.serialize(serializer)
    }
}
