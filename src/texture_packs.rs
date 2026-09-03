use crate::consts;
use crate::resources::TexturePackSettings;
use macros::dir_structure_as_enum;

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
