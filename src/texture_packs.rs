use crate::consts;
use bevy::prelude::Resource;
use macros::dir_structure_as_enum;

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
    pub fn get_asset_path(&self, asset: TexturePackAssets) -> String {
        format!("{}/{}", self.base_path, asset.get_path())
    }
}

dir_structure_as_enum!(TexturePackAssets, "assets/texture_packs/default");
