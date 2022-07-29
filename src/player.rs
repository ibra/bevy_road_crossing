use crate::{loader::SpriteAssets, GameState};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_player)
                .with_system(spawn_camera),
        );
    }
}

#[derive(Component)]
pub struct Player {
    speed: f32,
}

fn spawn_player(mut commands: Commands, textures: Res<SpriteAssets>) {
    let player_sprite = Sprite {
        color: Color::rgb(247.0, 102.0, 94.0),
        ..default()
    };

    commands
        .spawn_bundle(SpriteBundle {
            sprite: player_sprite,
            texture: textures.square.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..Default::default()
        })
        .insert(Player { speed: 4. });
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn move_player(time: Res<Time>) {}
