use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};

pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        AssetLoader::new(GameState::Loading)
            .with_collection::<FontAssets>()
            .continue_to_state(GameState::Menu)
            .build(app);
    }
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/RobotoBold.ttf")]
    pub roboto_bold: Handle<Font>,
    #[asset(path = "fonts/Roboto.ttf")]
    pub roboto_regular: Handle<Font>,
}
