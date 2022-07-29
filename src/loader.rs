use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};

pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        AssetLoader::new(GameState::Loading)
            .with_collection::<FontAssets>()
            .with_collection::<SpriteAssets>()
            .continue_to_state(GameState::Menu)
            .build(app);
    }
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/roboto_bold.ttf")]
    pub roboto_bold: Handle<Font>,
    #[asset(path = "fonts/roboto.ttf")]
    pub roboto_regular: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct SpriteAssets {
    #[asset(path = "sprites/square.png")]
    pub square: Handle<Image>,
}
