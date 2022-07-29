use crate::GameState;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_player)
                .with_system(spawn_cam),
        );
    }
}

fn spawn_player() {
    println!("Spawning Player")
}

fn spawn_cam() {}
