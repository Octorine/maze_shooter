use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use maze::Maze;
use std::f32::consts::PI;
mod input;
mod level;
mod player;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(level::Level::default())
        .add_plugins(leafwing_input_manager::prelude::InputManagerPlugin::<
            input::Action,
        >::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Update, input::move_player)
        .add_systems(Update, input::move_camera)
        .run();
}
