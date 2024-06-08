use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

use crate::enemy::{Enemy, EnemyCounts};

const BULLET_SPEED: f32 = 20.0;

#[derive(Component)]
pub struct Bullet {}

pub fn spawn_bullet(
    commands: &mut Commands,
    assets: ResMut<AssetServer>,
    dir: Vec3,
    base: &Transform,
) {
    let mut bullet_pos = base.translation + dir.clone() * 1.0;
    commands
        .spawn(SceneBundle {
            scene: assets.load("bullet.gltf#Scene0"),
            transform: Transform::from_translation(bullet_pos),
            ..Default::default()
        })
        .insert(Bullet {})
        .insert(LinearVelocity(dir * BULLET_SPEED))
        .insert(RigidBody::Kinematic)
        .insert(Collider::ball(0.1))
        .insert(Sensor);
}

pub fn hit_bullet(
    mut commands: Commands,
    query: Query<(&Bullet, Entity, &CollidingEntities)>,
    enemy_query: Query<(&Enemy, Entity)>,
    mut enemy_counts: ResMut<EnemyCounts>,
) {
    for (_bullet, bullet_entity, colliders) in query.iter() {
        if colliders.len() > 0 {
            commands.entity(bullet_entity).despawn_recursive();
            for (_, enemy_entity) in enemy_query.iter().filter(|(_, ee)| colliders.contains(ee)) {
                commands.entity(enemy_entity).despawn_recursive();
                enemy_counts.killed += 1;
            }
        }
    }
}
