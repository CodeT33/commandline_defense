use crate::consts;
use crate::ecs_elements::resources::TexturePackSettings;
use bevy::asset::AssetServer;
use bevy::image::{ImageLoaderSettings, ImageSampler};
use bevy::prelude::{Handle, Image};
use macros::dir_structure_as_enum;

impl Default for TexturePackSettings {
    fn default() -> Self {
        Self { base_path: consts::BASE_TEXTURE_PACK_PATH.to_owned() }
    }
}

impl TexturePackSettings {
    pub(crate) fn get_asset_path(&self, asset: TexturePackAssets) -> String {
        format!("{}/{}", self.base_path, asset.get_path())
    }

    pub(crate) fn load_nearest(
        &self, asset_server: &AssetServer, asset: TexturePackAssets,
    ) -> Handle<Image> {
        asset_server
            .load_builder()
            .with_settings(|settings: &mut ImageLoaderSettings| {
                settings.sampler = ImageSampler::nearest();
            })
            .load(self.get_asset_path(asset))
    }
}

dir_structure_as_enum!(TexturePackAssets, "assets/texture_packs/default");
