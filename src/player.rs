use crate::character_controller as cc;
use crate::fps::ShowFps;
use crate::input;
use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

const turn_speed: f32 = 10.0 * 2.0 * std::f32::consts::PI / 360.0;
const max_ammunition: usize = 6;
const regen_time: f32 = 1.0;
#[derive(Component)]
pub struct Player {
    pub ammunition: usize,
    pub last_shot_time: Option<f32>,
    pub aim: f32,
}

pub fn spawn_player(mut commands: &mut Commands, assets: &ResMut<AssetServer>, x: f32, y: f32) {
    commands
        .spawn(SceneBundle {
            scene: assets.load("Robot.gltf#Scene0"),
            transform: Transform::from_xyz(x, 1.5, y),
            ..Default::default()
        })
        .insert(input::input_bundle())
        .insert(Player {
            ammunition: max_ammunition,
            last_shot_time: None,
            aim: 0.0,
        })
        .insert(LinearVelocity(Vec3::new(0.0, 0.0, 0.0)))
        .insert(cc::CharacterControllerBundle::new(
            Collider::ball(0.5),
            Vec3::new(0.0, -1.0, 0.0),
        ));
}
pub fn update_player_ui(
    mut txt_query: Query<&mut Text>,
    player_query: Query<&Player>,
    diagnostics: Res<DiagnosticsStore>,
    show_fps: Res<ShowFps>,
) {
    let p = player_query.single();
    let mut txt = txt_query.single_mut();
    if show_fps.0 {
        let fps;
        if let Some(value) = diagnostics
            .get(FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
            .map(|smoothed| smoothed.round())
        {
            fps = value.to_string();
        } else {
            fps = String::from("Unknown");
        }
        txt.sections[0].value = format!("{} Bullets\t {} FPS", p.ammunition, fps);
    } else {
        txt.sections[0].value = format!("{} Bullets", p.ammunition);
    }
}
pub fn regen_ammo(time: Res<Time>, mut query: Query<&mut Player>) {
    let mut player = query.single_mut();
    let elapsed = time.elapsed_seconds();

    if player.ammunition < max_ammunition {
        player.last_shot_time = match player.last_shot_time {
            None => Some(elapsed),
            e => e,
        };
        if elapsed - player.last_shot_time.unwrap() > regen_time {
            player.last_shot_time = Some(elapsed);
            player.ammunition += 1;
        }
    } else {
        player.last_shot_time = None;
    }
}
pub fn spawn_player_ui(mut commands: Commands, query: Query<&Player>) {
    //    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        TextBundle::from_section(
            "Bullets: 6",
            TextStyle {
                font_size: 30.0,
                ..default()
            },
        )
        .with_style(Style {
            margin: UiRect::all(Val::Px(5.)),
            ..default()
        }),
        // Because this is a distinct label widget and
        // not button/list item text, this is necessary
        // for accessibility to treat the text accordingly.
        Label,
    ));
}
