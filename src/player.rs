use crate::{loader::SpriteAssets, GameState};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_player)
                .with_system(spawn_camera),
        )
        .add_system(move_player);
    }
}

#[derive(Component)]
pub struct Player {
    speed: f32,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Action {
    Up,
    Down,
    Left,
    Right,
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
        .insert_bundle(InputManagerBundle::<Action> {
            action_state: ActionState::default(),
            input_map: InputMap::new([(KeyCode::Up, Action::Up), (KeyCode::Down, Action::Down)]),
        })
        .insert(Player { speed: 4. });
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn move_player(time: Res<Time>) {}
