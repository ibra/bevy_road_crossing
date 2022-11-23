use crate::loader::FontAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy::text::TextStyle;

pub struct GameInterfacePlugin;

impl Plugin for GameInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(setup_game_interface),
        );
    }
}

fn setup_game_interface(mut commands: Commands, font: Res<FontAssets>) {
    commands.spawn_bundle(TextBundle::from_section(
        "0",
        TextStyle {
            font: font.roboto_bold.clone(),
            font_size: 100.0,
            color: Color::DARK_GRAY,
        },
    ));
}
