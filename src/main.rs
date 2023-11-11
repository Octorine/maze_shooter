use bevy::prelude::*;
use maze::Maze;
use std::f32::consts::PI;

mod level;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(level::Level::default())
        .run();
}
