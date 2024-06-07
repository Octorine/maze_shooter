use bevy::{
    core_pipeline::core_3d::Camera3dDepthLoadOp, diagnostic::FrameTimeDiagnosticsPlugin, prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_3d::{plugins::debug::PhysicsDebugPlugin, prelude::*};
use fps::ShowFps;
use leafwing_input_manager::prelude::InputManagerPlugin;
use oxidized_navigation::{NavMeshAffector, NavMeshSettings, OxidizedNavigationPlugin};
mod bullet;
mod character_controller;
mod enemy;
mod fps;
mod input;
mod level;
mod player;

fn main() {
    App::new()
        .insert_resource(ShowFps(false))
        .add_plugins((
            DefaultPlugins,
            //WorldInspectorPlugin::new(),
            FrameTimeDiagnosticsPlugin,
            level::LevelPlugin,
            InputManagerPlugin::<input::Action>::default(),
            OxidizedNavigationPlugin::<Collider>::new(self::NavMeshSettings {
                cell_width: 0.325,
                cell_height: 0.1625,
                tile_width: 100,
                world_half_extents: 160.0,
                world_bottom_bound: -1.0,
                max_traversable_slope_radians: (39.9_f32).to_radians(),
                walkable_height: 20,
                walkable_radius: 1,
                step_height: 3,
                min_region_area: 1,
                merge_region_area: 500,
                max_edge_length: 200,
                max_contour_simplification_error: 1.1,
                max_tile_generation_tasks: Some(100),
            }),
            oxidized_navigation::debug_draw::OxidizedNavigationDebugDrawPlugin,
            PhysicsPlugins::default(),
            character_controller::CharacterControllerPlugin,
        ))
        .add_systems(Startup, player::spawn_player_ui)
        .add_systems(
            Update,
            (
                bullet::hit_bullet,
                enemy::move_enemy,
                input::fire_gun,
                input::move_camera,
                input::move_player,
                input::toggle_fps,
                player::regen_ammo,
                player::update_player_ui,
            ),
        )
        .run();
}
