use crate::input;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
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
        .insert(KinematicCharacterController {
            slide: true,
            ..Default::default()
        });
}
