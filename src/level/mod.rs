use std::f32::consts::PI;

use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::{ecs::system::Commands, prelude::ResMut};
use bevy_rapier3d::prelude::*;
#[derive(Default)]
pub struct Level;
#[derive(Resource)]
pub struct Walls(Handle<Gltf>);

impl Plugin for Level {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: ResMut<AssetServer>,
) {
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 150.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
    });

    // plane
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(500.0).into()),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.2, 1.0, 0.2),
                perceptual_roughness: 0.8,
                ..default()
            }),
            ..default()
        })
        .insert(RigidBody::Fixed)
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
                y: -0.75,
                z: 1.0,
            }
            .normalize(),
            Vec3::Y,
        ),
        ..default()
    });
    spawn_walls(commands, assets);
}

fn spawn_walls(commands: Commands, assets: ResMut<AssetServer>) -> () {
    let mut maze = maze::Maze::new(30, 20);

    maze.add_walls();
    let mut ms = WallSpawner::new(commands, maze.height, maze.width, assets);

    // Draw the top row

    //   ms.draw_vertical_wall(0, 0);
    for i in 0..maze.width {
        ms.draw_horizontal_wall(i, 0);
        ms.draw_post(i, 0);
    }
    ms.draw_vertical_wall(maze.width, 0);

    for j in 0..maze.height {
        // Draw the bars.  One at the beginning, one at
        // the end, and one wherever there's an edge between
        // neighboring cells.
        ms.draw_vertical_wall(0, j);

        for i in 0..maze.width - 1 {
            let mut edges = 0;

            if maze.has_edge((i, j), (i + 1, j)) {
                edges += 1;
                ms.draw_vertical_wall(i + 1, j);
            }
            if maze.has_edge((i, j), (i, j + 1)) {
                edges += 1;
                ms.draw_horizontal_wall(i, j + 1);
            }
            if i > 0 && maze.has_edge((i, j + 1), (i + 1, j + 1)) {
                edges += 1;
            }
            if j > 0 && maze.has_edge((i + 1, j), (i + 1, j + 1)) {
                edges += 1;
            }
            if edges > 1 || i == 0 || j == 0 || i == maze.width - 1 || j == maze.height - 1 {
                ms.draw_post(i + 1, j + 1);
            }
        }
        ms.draw_post(maze.width, j);
        ms.draw_post(0, j);
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
    wall_height: f32,
    wall_length: f32,
    wall_thickness: f32,
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
        let wall_length = 4.0;
        let wall_height = 6.0;
        let wall_thickness = 1.0;
        let wall_scene = assets.load("Walls.gltf#Scene3");
        let post_scene = assets.load("Walls.gltf#Scene2");
        WallSpawner {
            height,
            width,
            wall_height,
            wall_length,
            wall_thickness,
            commands,
            wall_scene,
            post_scene,
        }
    }
    fn draw_post(&mut self, x: usize, y: usize) {
        let maze_width =
            self.width as f32 * (self.wall_length + self.wall_thickness) + self.wall_thickness;
        let maze_height =
            self.height as f32 * (self.wall_length + self.wall_thickness) + self.wall_thickness;
        self.commands.spawn(SceneBundle {
            scene: self.post_scene.clone(),
            transform: Transform::from_xyz(
                maze_width / -2.0
                    + x as f32 * (self.wall_thickness + self.wall_length)
                    + self.wall_thickness * 0.5,
                self.wall_height / 2.0,
                maze_height / -2.0
                    + (self.wall_thickness + self.wall_length) * y as f32
                    + self.wall_thickness * 0.5,
            ),
            ..Default::default()
        });
    }
    fn draw_horizontal_wall(&mut self, x: usize, y: usize) {
        let maze_width =
            self.width as f32 * (self.wall_length + self.wall_thickness) + self.wall_thickness;
        let maze_height =
            self.height as f32 * (self.wall_length + self.wall_thickness) + self.wall_thickness;
        self.commands
            .spawn(SceneBundle {
                scene: self.wall_scene.clone(),
                transform: Transform::from_xyz(
                    maze_width / -2.0
                        + self.wall_thickness
                        + x as f32 * (self.wall_thickness + self.wall_length)
                        + self.wall_length / 2.0,
                    self.wall_height / 2.0,
                    maze_height / -2.0
                        + (self.wall_thickness + self.wall_length) * y as f32
                        + self.wall_thickness * 0.5,
                ),
                ..Default::default()
            })
            .insert(RigidBody::Fixed)
            .insert(Collider::cuboid(
                self.wall_length,
                self.wall_height,
                self.wall_thickness,
            ));
    }
    pub fn draw_vertical_wall(&mut self, x: usize, y: usize) {
        let maze_width =
            self.width as f32 * (self.wall_length + self.wall_thickness) + self.wall_thickness;
        let maze_height =
            self.height as f32 * (self.wall_length + self.wall_thickness) + self.wall_thickness;
        self.commands
            .spawn(SceneBundle {
                scene: self.wall_scene.clone(),
                transform: Transform::from_xyz(
                    maze_width / -2.0
                        + x as f32 * (self.wall_thickness + self.wall_length)
                        + self.wall_thickness * 0.5,
                    self.wall_height / 2.0,
                    maze_height / -2.0
                        + self.wall_length / 2.0
                        + self.wall_thickness
                        + (self.wall_thickness + self.wall_length) * y as f32,
                )
                .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, PI / 2.0, 0.0)),
                ..Default::default()
            })
            .insert(RigidBody::Fixed)
            .insert(Collider::cuboid(
                self.wall_length,
                self.wall_height,
                self.wall_thickness,
            ));
    }
}
