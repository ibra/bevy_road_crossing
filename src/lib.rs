mod loader;
mod menu;
mod player;

use bevy::prelude::{App, Plugin};
use loader::LoaderPlugin;
use menu::MenuPlugin;
use player::PlayerPlugin;

pub struct GamePlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Loading,
    Menu,
    Playing,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_plugin(LoaderPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(PlayerPlugin);
    }
}
