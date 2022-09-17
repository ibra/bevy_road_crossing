use crate::{loader::SpriteAssets, GameState};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_player))
            .add_system(move_player);
    }
}

#[derive(Component)]
pub struct Player {
    speed: f32,
}

fn spawn_player(mut commands: Commands, textures: Res<SpriteAssets>) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_linear(1., 0.4, 0.36),
                ..default()
            },
            texture: textures.square.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..Default::default()
        })
        .insert(Player { speed: 500. });
}

fn move_player(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    for (player, mut transform) in query.iter_mut() {
        let mut direction = Vec2::new(0., 0.);

        if keyboard_input.just_pressed(KeyCode::A) {
            direction.x -= 1.;
        }
        if keyboard_input.just_pressed(KeyCode::D) {
            direction.x += 1.;
        }
        if keyboard_input.just_pressed(KeyCode::W) {
            direction.y += 1.;
        }

        let translation = &mut transform.translation;
        translation.x += time.delta_seconds() * direction.x * player.speed;
        translation.y += time.delta_seconds() * direction.y * player.speed;

        translation.x = translation.x.min(380.0).max(-380.0);
        translation.y = translation.y.min(480.0).max(-480.0);
    }
}
