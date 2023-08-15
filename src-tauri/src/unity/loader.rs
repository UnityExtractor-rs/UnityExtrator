use std::io::Read;

use unity_rs::{
    classes::{TextAsset, Texture2D},
    ClassID, Env, Sprite,
};

use super::{
    dto::{Loadable, LoadedPyaload, StoreUnityBoundle},
    loaded_object::LoadedObject,
    UnityResult,
};

impl Loadable for Texture2D {
    fn icon() -> &'static str {
        "mdi-image"
    }
    fn name(&self) -> String {
        self.name.to_owned()
    }

    fn meta(&self) -> Vec<String> {
        vec![
            format!("{:?}", self.format),
            format!("({} * {})", self.width, self.height),
        ]
    }

    fn load_preview(&self) -> super::UnityResult<LoadedPyaload> {
        let img = self.decode_image()?.value().clone();
        Ok(LoadedPyaload::Image(img))
    }
}

impl Loadable for TextAsset {
    fn icon() -> &'static str {
        "mdi-text-box"
    }
    fn name(&self) -> String {
        self.name.clone()
    }

    fn meta(&self) -> Vec<String> {
        vec![format!("script len:{}", self.script.len())]
    }

    fn load_preview(&self) -> UnityResult<LoadedPyaload> {
        let payload = self
            .script_string()
            .map(|s| LoadedPyaload::Text(s.to_string()))
            .or_else(|_| Ok::<_, unity_rs::UnityError>(LoadedPyaload::Raw(self.script.clone())))?;
        Ok(payload)
    }
}

impl Loadable for Sprite<'_> {
    fn icon() -> &'static str {
        "mdi-image-auto-adjust"
    }
    fn name(&self) -> String {
        self.name.clone()
    }

    fn meta(&self) -> Vec<String> {
        vec![format!(
            "x: {:.02}, y: {:.02}, width: {:.02}, height: {:.02}",
            self.rect.x, self.rect.y, self.rect.w, self.rect.h
        )]
    }

    fn load_preview(&self) -> super::UnityResult<LoadedPyaload> {
        let img = self.decode_image()?.clone();
        Ok(LoadedPyaload::Image(img))
    }
}

impl StoreUnityBoundle {
    pub fn load(path: &str, name: &str) -> UnityResult<Self> {
        let mut env = Env::new();
        let mut file = std::fs::OpenOptions::new().read(true).open(path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;

        env.load_from_slice(&buf)?;

        let mut objects = Vec::new();
        for obj in env.objects().filter_map(|obj| match obj.class() {
            c @ ClassID::Texture2D => Some(
                obj.read::<Texture2D>()
                    .map_err(Into::into)
                    .and_then(|v| LoadedObject::new(obj.info.path_id, v).map(|obj| (c, obj))),
            ),
            c @ ClassID::TextAsset => Some(
                obj.read::<TextAsset>()
                    .map_err(Into::into)
                    .and_then(|v| LoadedObject::new(obj.info.path_id, v).map(|obj| (c, obj))),
            ),
            c @ ClassID::Sprite => Some(
                obj.read::<Sprite>()
                    .map_err(Into::into)
                    .and_then(|v| LoadedObject::new(obj.info.path_id, v).map(|obj| (c, obj))),
            ),
            _ => None,
        }) {
            let obj = obj?;
            objects.push(obj)
        }

        Ok(Self {
            origin: env,
            objects,
            location: path.to_string(),
            name: name.to_string(),
        })
    }
}
