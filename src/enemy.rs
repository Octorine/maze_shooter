use crate::character_controller as cc;
use crate::input;
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

#[derive(Component)]
pub struct Enemy;
pub fn spawn_enemy(mut commands: &mut Commands, assets: &ResMut<AssetServer>, x: f32, y: f32) {
    commands
        .spawn(SceneBundle {
            scene: assets.load("Bug.glb#Scene0"),
            transform: Transform::from_xyz(x, 1.5, y),
            ..Default::default()
        })
        .insert(input::input_bundle())
        .insert(Enemy)
        .insert(LinearVelocity(Vec3::new(0.0, 0.0, 0.0)))
        .insert(cc::CharacterControllerBundle::new(
            Collider::ball(0.5),
            Vec3::new(0.0, -1.0, 0.0),
        ));
}
