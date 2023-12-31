use crate::player;
use bevy::{prelude::*, time::TimePlugin};
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Action {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    AimLeft,
    AimRight,
    AimUp,
    AimDown,
    Shoot,
}

pub fn input_bundle() -> InputManagerBundle<Action> {
    InputManagerBundle {
        action_state: ActionState::default(),
        // Describes how to convert from player inputs into those actions
        input_map: InputMap::new([
            (KeyCode::W, Action::MoveUp),
            (KeyCode::A, Action::MoveLeft),
            (KeyCode::S, Action::MoveDown),
            (KeyCode::D, Action::MoveRight),
        ]),
    }
}

pub fn move_player(
    mut query: Query<(&ActionState<Action>, &mut KinematicCharacterController)>,
    t: Res<Time>,
) {
    let (action_state, mut player) = query.single_mut();

    // Each action has a button-like state of its own that you can check
    let mut xlat = Vec3::new(0.0, 0.0, 0.0);
    let speed = 5.0;
    if action_state.pressed(Action::MoveUp) {
        xlat.z += 1.0;
    }
    if action_state.pressed(Action::MoveLeft) {
        xlat.x += 1.0;
    }
    if action_state.pressed(Action::MoveDown) {
        xlat.z -= 1.0;
    }
    if action_state.pressed(Action::MoveRight) {
        xlat.x -= 1.0;
    }
    player.translation = Some(xlat.normalize_or_zero() * speed * t.delta_seconds());
}

pub fn move_camera(
    mut camera_query: Query<(&mut Transform, &Camera3d), Without<KinematicCharacterController>>,
    player_query: Query<(&Transform, &KinematicCharacterController)>,
    time: Res<Time>,
) {
    let (player_transform, player) = player_query.single();
    let (mut camera_transform, camera) = camera_query.single_mut();
    let delta = time.delta_seconds();
    let threshold = 3.0;
    let camera_speed = 6.0;
    let mut camera_projection = camera_transform.translation.clone();
    camera_projection.y = 0.0;
    let mut player_projection = player_transform.translation.clone();
    player_projection.y = 0.0;

    let distance = player_projection - camera_projection;
    if distance.length() > threshold {
        camera_transform.translation += distance / distance.length() * camera_speed * delta;
    }
}
