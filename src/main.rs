#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(private_interfaces)]

use std::process::Command;

use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::log::tracing_subscriber::filter;
use bevy::window::{PrimaryWindow, WindowMode, WindowResolution};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::{prelude::*, transform, window};
use rand::random;

use Map::MapPlugin;
mod Map;
use Turret::components::Turrets;
use Turret::{TurretPlugin, BULLET_DAMAGE_F, BULLET_SPEED_F, COOLDOWN_F, REACH_F, TURRET_SIZE};
mod Turret;

use Enemies::EnemiesPlugin;
use Targeting::TargetingPlugin;

use Money::MoneyPlugin;
mod Money;

mod Enemies;
mod Targeting;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "RustTD".into(),
                name: Some("bevy.app".into()),
                resizable: false,
                mode: WindowMode::BorderlessFullscreen,
                resolution: WindowResolution::new(1920.0, 1080.0).with_scale_factor_override(1.0),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(TurretPlugin {
            state: GameState::Playing
        })
        .add_plugins(EnemiesPlugin {
            state: GameState::Playing
        })
        .add_plugins(TargetingPlugin {
            state: GameState::Playing
        })
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, pause_system)
        //.add_systems(Startup, spawn_test_turret)
        .add_plugins(MapPlugin)
        .add_plugins(MoneyPlugin)
        .insert_state(GameState::Playing)
        .run();
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}



// #[derive(Component)]
// pub struct Turrets {
//     pub dir_look: Vec3,
// }

#[derive(Component)]
pub struct DebugText;

// pub fn spawn_test_turret(
//     mut commands: Commands,
//     window_query: Query<&Window, With<PrimaryWindow>>,
//     asset_server: Res<AssetServer>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     let window = window_query.get_single().unwrap();

//     let turret_x = window.width() / 4.0 ;
//     let turret_y = (window.height() / 2.0) - 100.0;

//     commands.spawn((
//         SpriteBundle {
//             transform: Transform::from_xyz(turret_x, turret_y, 2.0),
//             texture: asset_server.load("sprites/kenney_tower-defense-top-down/PNG/Default size/towerDefense_tile249.png"),
//             sprite: Sprite {
//                 custom_size: Some(Vec2::new(TURRET_SIZE, TURRET_SIZE)),
//                 ..default()
//             },
//             ..default()
//         },
//         Turrets {
//             dir_look: Vec3::new(0.0, 0.0, 0.0),
//             b_speed: BULLET_SPEED_F,
//             cooldown: COOLDOWN_F,
//             cooldown_max: COOLDOWN_F,
//             reach: REACH_F,
//             b_damage: BULLET_DAMAGE_F
//         }
//     ));

//     commands.spawn(MaterialMesh2dBundle {
//         transform: Transform::from_xyz(turret_x, turret_y, 0.0),
//         mesh: Mesh2dHandle(meshes.add( Circle { radius: REACH_F })),
//         material: materials.add(Color::rgba(0.0, 0.0, 1.0, 0.1)),
//         ..default()
//     });

//     commands.spawn((
//         // Create a TextBundle that has a Text with a single section.
//         TextBundle::from_section(
//             // Accepts a `String` or any type that converts into a `String`, such as `&str`
//             "hello bevy!",
//             TextStyle {
//                 // This font is loaded and will be used instead of the default font.
//                 font_size: 50.0,
//                 ..default()
//             },
//         ) // Set the justification of the Text
//         .with_text_justify(JustifyText::Center)
//         // Set the style of the TextBundle itself.
//         .with_style(Style {
//             position_type: PositionType::Absolute,
//             bottom: Val::Px(5.0),
//             right: Val::Px(5.0),
//             ..default()
//         }),
//         DebugText,
//     ));

// }

#[derive(States, Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    Playing,
    Paused,
}

pub fn pause_system(
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>
) {
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        match state.get() {
            GameState::Playing => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::Playing)
        }
    }
}