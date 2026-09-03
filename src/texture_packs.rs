use crate::consts;
use bevy::prelude::Resource;

#[derive(Resource)]
pub struct TexturePackSettings {
    base_path: String,
}

impl Default for TexturePackSettings {
    fn default() -> Self {
        Self { base_path: consts::BASE_TEXTURE_PACK_PATH.to_owned() }
    }
}

impl TexturePackSettings {
    pub fn get_asset_path(&self, path_extension: &str) -> String {
        format!("{}/{}", self.base_path, path_extension)
    }
}
