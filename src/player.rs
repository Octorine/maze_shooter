use crate::input;
use bevy::prelude::*;
use bevy_rapier3d::{na::RealField, prelude::*};

const turn_speed: f32 = 10.0 * 2.0 * std::f32::consts::PI / 360.0;

#[derive(Component)]
pub struct Player {
    pub ammunition: usize,
    pub regeneration: f32,
    pub last_shot_time: Option<f32>,
    pub aim: f32,
}

pub fn spawn_player(commands: &mut Commands, assets: &ResMut<AssetServer>, x: f32, y: f32) {
    commands
        .spawn(SceneBundle {
            scene: assets.load("Robot.gltf#Scene0"),
            transform: Transform::from_xyz(x, 2.0, y),
            ..Default::default()
        })
        .insert(RigidBody::KinematicVelocityBased)
        .insert(Collider::ball(0.5))
        .insert(input::input_bundle())
        .insert(Player {
            ammunition: 6,
            regeneration: 0.0,
            last_shot_time: None,
            aim: 0.0,
        })
        .insert(KinematicCharacterController {
            slide: true,
            ..Default::default()
        });
}
