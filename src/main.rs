use bevy::prelude::{App, ClearColor, Color, WindowDescriptor};
use bevy::DefaultPlugins;

use bevy_road_crossing::GamePlugin;
use leafwing_input_manager::plugin::InputManagerPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(1., 1., 1.)))
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 600.,
            title: "Road Crossing".to_string(), // ToDo
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .add_plugin(InputManagerPlugin::<Action>::default())
        .run();
}
