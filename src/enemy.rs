use crate::character_controller as cc;
use crate::input;
use crate::player;
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use oxidized_navigation::debug_draw::DrawPath;
use oxidized_navigation::query::find_path;
use oxidized_navigation::NavMesh;
use oxidized_navigation::NavMeshSettings;
const ENEMY_SPEED: f32 = 10.0;
const ENEMY_PATH_PERIOD: f32 = 0.4;
pub const enemies_to_spawn: i32 = 40;

#[derive(Resource)]
pub struct EnemyCounts {
    pub count: i32,
    pub killed: i32,
}

#[derive(Component)]
pub struct Enemy {
    current_path: Vec3,
    last_path_set: f32,
}
pub fn spawn_enemy(
    mut commands: &mut Commands,
    assets: &ResMut<AssetServer>,
    time: &Res<Time>,
    x: f32,
    y: f32,
) {
    commands
        .spawn(SceneBundle {
            scene: assets.load("Bug.glb#Scene0"),
            transform: Transform::from_xyz(x, 1.5, y),
            ..Default::default()
        })
        .insert(input::input_bundle())
        .insert(Enemy {
            current_path: Vec3::new(0.0, 0.0, 0.0),
            last_path_set: time.elapsed_seconds(),
        })
        .insert(LinearVelocity(Vec3::new(0.0, 0.0, 0.0)))
        //.insert(DrawPath {
        //    color: Color::Rgba {
        //        red: 0.0,
        //        green: 0.0,
        //        blue: 1.0,
        //        alpha: 1.0,
        //    },
        //    pulled_path: vec![],
        //    timer: None,
        //})
        .insert(cc::CharacterControllerBundle::new(
            Collider::ball(0.75),
            Vec3::new(0.0, -1.0, 0.0),
        ));
}
pub fn move_enemy(
    player_query: Query<(&player::Player, &Transform), Without<Enemy>>,
    mut enemy_query: Query<
        (&mut Enemy, &mut LinearVelocity, &mut Transform),
        Without<player::Player>,
    >,
    navigation: Res<NavMesh>,
    time: Res<Time>,
    settings: Res<NavMeshSettings>,
) {
    let (_, player_xform) = player_query.single();
    let binding = navigation.get();
    if let tileset = binding.try_read() {
        for (mut enemy, mut velocity, mut enemy_xform) in enemy_query.iter_mut() {
            if time.elapsed_seconds() - enemy.last_path_set > ENEMY_PATH_PERIOD {
                let enemy_pos = enemy_xform.translation;
                let player_pos = player_xform.translation;
                let direct = (player_pos.clone() - enemy_pos.clone()).normalize();
                let heading = match &tileset {
                    Ok(tiles) => {
                        //println!("Tileset contains {} tiles.", tiles.tiles.len());
                        match find_path(&tiles, &settings, enemy_pos, player_pos, None, None) {
                            Ok(path) => {
                                let goal = if path.len() > 1 { path[1] } else { path[0] };
                                let mut next = goal - enemy_pos.clone();
                                next.y = 0.0;
                                next.normalize()
                                //println!("Path Found: {:?}", path);
                                //println!("Enemy: {:?}", &enemy_pos);
                                //println!("Player: {:?}", &player_pos);
                            }
                            Err(e) => {
                                println!("Path Error: {:?}", e);
                                //println!(
                                //    "Tileset contains {} tiles at {}.",
                                //    tiles.tiles.len(),
                                //    time.elapsed_seconds()
                                //);
                                direct
                            }
                        }
                    }
                    Err(e) => {
                        println!("Navigation Tiles Error: {:?}", e);
                        direct
                    }
                };
                enemy.last_path_set = time.elapsed_seconds();
                enemy.current_path = heading;
                velocity.0 = enemy.current_path * ENEMY_SPEED;
            }
            let heading = enemy.current_path;
            if heading.x.abs() > 0.0 {
                let heading_angle = heading.x.atan2(heading.z);
                enemy_xform.rotation = Quat::from_rotation_y(heading_angle);
            }
        }
    };
}
