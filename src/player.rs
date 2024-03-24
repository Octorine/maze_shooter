use crate::character_controller as cc;
use crate::input;
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
const turn_speed: f32 = 10.0 * 2.0 * std::f32::consts::PI / 360.0;

#[derive(Component)]
pub struct Player {
    pub ammunition: usize,
    pub regeneration: f32,
    pub last_shot_time: Option<f32>,
    pub aim: f32,
}

pub fn spawn_player(mut commands: &mut Commands, assets: &ResMut<AssetServer>, x: f32, y: f32) {
    commands
        .spawn(SceneBundle {
            scene: assets.load("Robot.gltf#Scene0"),
            transform: Transform::from_xyz(x, 1.5, y),
            ..Default::default()
        })
        .insert(input::input_bundle())
        .insert(Player {
            ammunition: 6,
            regeneration: 0.0,
            last_shot_time: None,
            aim: 0.0,
        })
        .insert(LinearVelocity(Vec3::new(0.0, 0.0, 0.0)))
        .insert(cc::CharacterControllerBundle::new(
            Collider::ball(0.5),
            Vec3::new(0.0, -1.0, 0.0),
        ));
}
