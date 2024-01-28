use std::f32::consts::PI;

use crate::player;
use bevy::{
    input::gamepad::{GamepadAxisChangedEvent, GamepadButtonChangedEvent, GamepadButtonInput},
    prelude::*,
    time::TimePlugin,
};
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::{prelude::*, user_input::InputKind};
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
    Aim,
    Move,
}

pub fn input_bundle() -> InputManagerBundle<Action> {
    InputManagerBundle {
        action_state: ActionState::default(),
        // Describes how to convert from player inputs into those actions
        input_map: InputMap::new([
            (InputKind::Keyboard(KeyCode::W), Action::MoveUp),
            (InputKind::Keyboard(KeyCode::A), Action::MoveLeft),
            (InputKind::Keyboard(KeyCode::S), Action::MoveDown),
            (InputKind::Keyboard(KeyCode::D), Action::MoveRight),
            (InputKind::Keyboard(KeyCode::Left), Action::AimLeft),
            (InputKind::Keyboard(KeyCode::Right), Action::AimRight),
            (InputKind::Keyboard(KeyCode::Up), Action::AimUp),
            (InputKind::Keyboard(KeyCode::Down), Action::AimDown),
            (InputKind::DualAxis(DualAxis::left_stick()), Action::Move),
            (InputKind::DualAxis(DualAxis::right_stick()), Action::Aim),
        ]),
    }
}

pub fn move_player(
    mut query: Query<(
        &ActionState<Action>,
        &mut KinematicCharacterController,
        &mut player::Player,
        &mut Transform,
    )>,
    t: Res<Time>,
) {
    let (action_state, mut playerController, mut player, mut xform) = query.single_mut();

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
    let joystick_move = action_state.axis_pair(Action::Move).unwrap_or_default();
    xlat.x -= joystick_move.x();
    xlat.z += joystick_move.y();
    playerController.translation = Some(xlat.normalize_or_zero() * speed * t.delta_seconds());
    let mut aim = xform.rotation;
    if action_state.pressed(Action::AimUp) {
        aim = Quat::from_rotation_y(0.5 * PI);
    } else if action_state.pressed(Action::AimDown) {
        aim = Quat::from_rotation_y(1.5 * PI);
    } else if action_state.pressed(Action::AimRight) {
        aim = Quat::from_rotation_y(0.0);
    } else if action_state.pressed(Action::AimLeft) {
        aim = Quat::from_rotation_y(PI);
    } else if action_state.axis_pair(Action::Aim).is_some() {
        let aim_joystick = action_state.axis_pair(Action::Aim).unwrap();
        let aim_x = aim_joystick.x();
        let aim_y = aim_joystick.y();
        let magnitude = (aim_x * aim_x + aim_y * aim_y).sqrt();
        if magnitude > 0.2 && aim_x.abs() > 0.0 {
            aim = Quat::from_rotation_y(aim_y.atan2(aim_x));
        }
    }
    player.aim = aim.to_euler(EulerRot::XYZ).1;
    xform.rotation = xform.rotation.slerp(aim, 0.1);
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
