use std::f32::consts::PI;

use bevy::prelude::*;
use maze::Maze;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 300.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
    });

    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(500.0).into()),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.2, 1.0, 0.2),
            perceptual_roughness: 0.8,
            ..default()
        }),
        ..default()
    });
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
    spawn_walls(commands, meshes, materials);
}

fn spawn_walls(
    commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) -> () {
    let mut maze = maze::Maze::new(30, 20);

    maze.add_walls();
    let mut ms = WallSpawner::new(commands, maze.height, maze.width, meshes, materials);

    // Draw the top row

    ms.draw_vertical_wall(0, 0);
    for i in 0..maze.width {
        ms.draw_horizontal_wall(i, 0);
    }
    ms.draw_vertical_wall(maze.width, 0);

    for j in 0..maze.height - 1 {
        // Draw the vertical bars.  One at the beginning, one at
        // the end, and one wherever there's an edge between
        // neighboring cells.
        ms.draw_vertical_wall(0, j + 1);

        for i in 0..maze.width - 1 {
            if maze.has_edge((i, j), (i + 1, j)) {
                ms.draw_vertical_wall(i + 1, j);
            }
            if maze.has_edge((i, j), (i, j + 1)) {
                ms.draw_horizontal_wall(i, j + 1);
            }
        }
        ms.draw_vertical_wall(maze.width, j + 1);
    }
    // Draw the bottom row
    //    ms.draw_vertical_wall(0, maze.height);
    for i in 0..maze.width {
        ms.draw_horizontal_wall(i, maze.height);
    }
    //  ms.draw_vertical_wall(maze.width, maze.height);
}
struct WallSpawner<'a, 'w, 'c> {
    height: usize,
    width: usize,
    wall_height: f32,
    wall_length: f32,
    wall_thickness: f32,
    commands: Commands<'w, 'c>,
    meshes: ResMut<'a, Assets<Mesh>>,
    mesh: Handle<Mesh>,
    materials: ResMut<'a, Assets<StandardMaterial>>,
}

impl<'a, 'w, 'c> WallSpawner<'a, 'w, 'c> {
    pub fn new(
        commands: Commands<'w, 'c>,
        height: usize,
        width: usize,
        mut meshes: ResMut<'a, Assets<Mesh>>,
        materials: ResMut<'a, Assets<StandardMaterial>>,
    ) -> WallSpawner<'a, 'w, 'c> {
        let wall_length = 10.0;
        let wall_height = 6.0;
        let wall_thickness = 1.0;
        let mesh = meshes.add(Mesh::from(shape::Box {
            min_x: -wall_length / 2.0,
            min_y: -wall_height / 2.0,
            min_z: -wall_thickness / 2.0,
            max_x: wall_length / 2.0,
            max_y: wall_height / 2.0,
            max_z: wall_thickness / 2.0,
        }));
        WallSpawner {
            height,
            width,
            wall_height,
            wall_length,
            wall_thickness,
            commands,
            meshes,
            mesh,
            materials,
        }
    }
    pub fn draw_horizontal_wall(&mut self, x: usize, y: usize) {
        let maze_width =
            self.width as f32 * self.wall_length + (1 + self.width) as f32 * self.wall_thickness;
        let maze_height =
            self.height as f32 * self.wall_length + (1 + self.height) as f32 * self.wall_thickness;
        self.commands.spawn(PbrBundle {
            mesh: self.mesh.clone(),
            material: self.materials.add(StandardMaterial {
                base_color: Color::rgb(0.95, 0.95, 0.95),
                unlit: false,
                ..default()
            }),

            transform: Transform::from_xyz(
                maze_width / -2.0
                    + self.wall_thickness
                    + x as f32 * (self.wall_thickness + self.wall_length)
                    + self.wall_length / 2.0,
                self.wall_height / 2.0,
                maze_height / -2.0
                    + self.wall_thickness / 2.0
                    + (self.wall_thickness + self.wall_length) * y as f32,
            ),
            ..default()
        });
    }
    pub fn draw_vertical_wall(&mut self, x: usize, y: usize) {
        let maze_width =
            self.width as f32 * self.wall_length + (1 + self.width) as f32 * self.wall_thickness;
        let maze_height =
            self.height as f32 * self.wall_length + (1 + self.height) as f32 * self.wall_thickness;
        self.commands.spawn(PbrBundle {
            mesh: self.mesh.clone(),
            material: self.materials.add(StandardMaterial {
                base_color: Color::rgb(0.95, 0.95, 0.95),
                unlit: false,
                ..default()
            }),

            transform: Transform::from_xyz(
                maze_width / -2.0
                    + self.wall_thickness / 2.0
                    + x as f32 * (self.wall_thickness + self.wall_length),
                self.wall_height / 2.0,
                maze_height / -2.0
                    + self.wall_length / 2.0
                    + self.wall_thickness
                    + (self.wall_thickness + self.wall_length) * y as f32,
            )
            .with_rotation(Quat::from_rotation_y(PI / 2.0)),
            ..default()
        });
    }
}
