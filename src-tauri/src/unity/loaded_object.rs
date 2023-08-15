use std::sync::Arc;

use super::{
    dto::{Loadable, LoadedPyaload},
    UnityResult,
};

pub struct LoadedObject {
    pub obj_id: i64,
    pub icon: &'static str,
    pub name: String,
    pub meta: Vec<String>,
    pub payload: LoadedPyaload,
}

impl LoadedObject {
    pub fn new<L: Loadable>(path_id: i64, loaded: L) -> UnityResult<Arc<Self>> {
        Ok(Arc::new(Self {
                    icon: L::icon(),
                    obj_id: path_id,
                    name: loaded.name(),
                    meta: loaded.meta(),
                    payload: loaded.load_preview()?,
                }))
    }
}
