use std::f32::consts::PI;

use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::{ecs::system::Commands, prelude::ResMut};
use bevy_xpbd_3d::prelude::*;
use oxidized_navigation::debug_draw::{DrawNavMesh, DrawPath};
use oxidized_navigation::NavMeshAffector;
use rand::Rng;

use crate::enemy::{self, EnemyCounts};

const wall_length: f32 = 4.0;
const wall_height: f32 = 6.0;
const wall_thickness: f32 = 1.0;

#[derive(Default)]
pub struct LevelPlugin;
#[derive(Resource)]
pub struct Walls(Handle<Gltf>);
#[derive(Component)]
pub struct Wall;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: ResMut<AssetServer>,
    time: Res<Time>,
    mut draw_mesh: ResMut<DrawNavMesh>,
    enemy_counts: ResMut<EnemyCounts>,
) {
    //draw_mesh.0 = true;
    let maze_width = 30;
    let maze_height = 20;
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(3.5, 25.0, 3.5).looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
    });
    crate::player::spawn_player(&mut commands, &assets, 3.5, 3.5);
    let mut rng = rand::thread_rng();
    for enemy_num in 1..enemy::enemies_to_spawn {
        let mut enemy_x: i32 = 0;
        let mut enemy_y: i32 = 0;
        while (enemy_x.abs() < 3 && enemy_y.abs() < 3) {
            enemy_x = rng.gen_range(-1 * maze_width as i32 / 2..maze_width as i32 / 2);
            enemy_y = rng.gen_range(-1 * maze_height as i32 / 2..maze_height as i32 / 2);
        }
        let enemy_x = enemy_x as f32 * (wall_length + wall_thickness);
        let enemy_y = enemy_y as f32 * (wall_length + wall_thickness);
        crate::enemy::spawn_enemy(&mut commands, &assets, &time, enemy_x + 3.5, enemy_y + 3.5);
    }
    // plane
    commands
        .spawn(PbrBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            mesh: meshes.add(shape::Plane::from_size(500.0).into()),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.2, 1.0, 0.2),
                perceptual_roughness: 0.8,
                ..default()
            }),
            ..default()
        })
        .insert(RigidBody::Static)
        .insert(NavMeshAffector)
        .insert(Collider::cuboid(500.0, 0.01, 500.0));
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 5000.0,
            color: Color::rgb(0.75, 0.75, 0.75),
            ..default()
        },
        transform: Transform::IDENTITY.looking_to(
            Vec3 {
                x: 1.0,
                y: -2.0,
                z: 1.0,
            }
            .normalize(),
            Vec3::Y,
        ),
        ..default()
    });
    spawn_walls(commands, assets, maze_width, maze_height);
}

fn spawn_walls(
    commands: Commands,
    assets: ResMut<AssetServer>,
    maze_width: usize,
    maze_height: usize,
) -> () {
    let mut maze = maze::Maze::new(maze_width, maze_height);
    maze.add_walls();
    let mut ms = WallSpawner::new(commands, maze.height, maze.width, assets);

    // Draw the top row

    //   ms.draw_vertical_wall(0, 0);
    for i in 0..maze.width {
        ms.draw_horizontal_wall(i, 0);
        ms.draw_post(i, 0);
    }
    ms.draw_post(maze.width, 0);

    for j in 0..maze.height {
        // Draw the bars.  One at the beginning, one at
        // the end, and one wherever there's an edge between
        // neighboring cells.
        ms.draw_vertical_wall(0, j);
        ms.draw_post(0, j);

        for i in 0..maze.width {
            let mut edges = 0;

            if maze.has_edge((i, j), (i + 1, j)) {
                edges += 1;
                ms.draw_vertical_wall(i + 1, j);
            }

            if maze.has_edge((i, j), (i, j + 1)) {
                edges += 1;
                ms.draw_horizontal_wall(i, j + 1);
            }
            if maze.has_edge((i, j + 1), (i + 1, j + 1)) {
                edges += 1;
            }
            if maze.has_edge((i + 1, j), (i + 1, j + 1)) {
                edges += 1;
            }
            if edges > 1 {
                ms.draw_post(i + 1, j + 1);
            }
        }
        ms.draw_post(maze.width, j);
        ms.draw_vertical_wall(maze.width, j);
    }
    // Draw the top row
    for i in 0..maze.width {
        ms.draw_horizontal_wall(i, maze.height);
        ms.draw_post(i, maze.height);
    }
    ms.draw_post(maze.width, maze.height);
}
struct WallSpawner<'w, 'c> {
    height: usize,
    width: usize,
    commands: Commands<'w, 'c>,
    wall_scene: Handle<Scene>,
    post_scene: Handle<Scene>,
}

impl<'w, 'c> WallSpawner<'w, 'c> {
    pub fn new(
        commands: Commands<'w, 'c>,
        height: usize,
        width: usize,
        assets: ResMut<AssetServer>,
    ) -> WallSpawner<'w, 'c> {
        let wall_scene = assets.load("Walls.gltf#Scene3");
        let post_scene = assets.load("Walls.gltf#Scene2");
        WallSpawner {
            height,
            width,
            commands,
            wall_scene,
            post_scene,
        }
    }
    fn draw_post(&mut self, x: usize, y: usize) {
        let maze_width = self.width as f32 * (wall_length + wall_thickness) + wall_thickness;
        let maze_height = self.height as f32 * (wall_length + wall_thickness) + wall_thickness;
        self.commands
            .spawn(SceneBundle {
                scene: self.post_scene.clone(),
                transform: Transform::from_xyz(
                    maze_width / -2.0
                        + x as f32 * (wall_thickness + wall_length)
                        + wall_thickness * 0.5,
                    wall_height / 2.0,
                    maze_height / -2.0
                        + (wall_thickness + wall_length) * y as f32
                        + wall_thickness * 0.5,
                ),
                ..Default::default()
            })
            .insert(RigidBody::Static)
            .insert(NavMeshAffector)
            .insert(Wall)
            .insert(Collider::cuboid(
                wall_thickness,
                wall_height,
                wall_thickness,
            ));
    }
    fn draw_horizontal_wall(&mut self, x: usize, y: usize) {
        let maze_width = self.width as f32 * (wall_length + wall_thickness) + wall_thickness;
        let maze_height = self.height as f32 * (wall_length + wall_thickness) + wall_thickness;
        self.commands
            .spawn(SceneBundle {
                scene: self.wall_scene.clone(),
                transform: Transform::from_xyz(
                    maze_width / -2.0
                        + wall_thickness
                        + x as f32 * (wall_thickness + wall_length)
                        + wall_length / 2.0,
                    wall_height / 2.0,
                    maze_height / -2.0
                        + (wall_thickness + wall_length) * y as f32
                        + wall_thickness * 0.5,
                ),
                ..Default::default()
            })
            .insert(RigidBody::Static)
            .insert(Wall)
            .insert(NavMeshAffector)
            .insert(Collider::cuboid(wall_length, wall_height, wall_thickness));
    }
    pub fn draw_vertical_wall(&mut self, x: usize, y: usize) {
        let maze_width = self.width as f32 * (wall_length + wall_thickness) + wall_thickness;
        let maze_height = self.height as f32 * (wall_length + wall_thickness) + wall_thickness;
        self.commands
            .spawn(SceneBundle {
                scene: self.wall_scene.clone(),
                transform: Transform::from_xyz(
                    maze_width / -2.0
                        + x as f32 * (wall_thickness + wall_length)
                        + wall_thickness * 0.5,
                    wall_height / 2.0,
                    maze_height / -2.0
                        + wall_length / 2.0
                        + wall_thickness
                        + (wall_thickness + wall_length) * y as f32,
                )
                .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, PI / 2.0, 0.0)),
                ..Default::default()
            })
            .insert(RigidBody::Static)
            .insert(Wall)
            .insert(NavMeshAffector)
            .insert(Collider::cuboid(wall_length, wall_height, wall_thickness));
    }
}
