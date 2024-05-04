use bevy::{core_pipeline::core_3d::Camera3dDepthLoadOp, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_3d::{plugins::debug::PhysicsDebugPlugin, prelude::*};
use leafwing_input_manager::prelude::InputManagerPlugin;
use maze::Maze;
use std::f32::consts::PI;
mod bullet;
mod character_controller;
mod input;
mod level;
mod player;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(level::LevelPlugin)
        .add_plugins(InputManagerPlugin::<input::Action>::default())
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(character_controller::CharacterControllerPlugin)
        .add_systems(Startup, player::spawn_player_ui)
        .add_systems(Update, input::move_player)
        .add_systems(Update, player::update_player_ui)
        .add_systems(Update, input::move_camera)
        .add_systems(Update, input::fire_gun)
        .add_systems(Update, bullet::hit_bullet)
        .add_systems(Update, player::regen_ammo)
        .run();
}
