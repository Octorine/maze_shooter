use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use maze::Maze;
use std::f32::consts::PI;
mod level;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(level::Level::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        //        .add_plugins(RapierDebugRenderPlugin::default())
        .run();
}
