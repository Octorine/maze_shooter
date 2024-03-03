use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_3d::{plugins::debug::PhysicsDebugPlugin, prelude::*};
use leafwing_input_manager::prelude::InputManagerPlugin;
use maze::Maze;
use std::f32::consts::PI;
mod character_controller;
mod input;
mod level;
mod player;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(level::LevelPlugin)
        .add_plugins(InputManagerPlugin::<input::Action>::default())
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(character_controller::CharacterControllerPlugin)
        .add_systems(Update, input::move_player)
        .add_systems(Update, input::move_camera)
        .run();
}
