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
use Turret::{TurretPlugin, BULLET_DAMAGE_F, BULLET_SPEED_F, COOLDOWN_F, REACH_F, TURRET_SIZE, TURRET_S_COST};
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
        .add_systems(Startup, keyinput_ui_setup)
        .add_systems(Update, pause_system)
        //.add_systems(Startup, spawn_test_turret)
        .add_plugins(MapPlugin)
        .add_plugins(MoneyPlugin)
        .insert_state(GameState::Paused)
        .run();
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn keyinput_ui_setup(mut commands: Commands) {

    // TOUCHES
    commands.spawn(
        TextBundle::from_section("TOUCHES", TextStyle {
            font_size: 30.0,
            ..default()
        }).with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            right: Val::Px(40.0),
            ..default()
        })
    );

    // Touche 1 -> Tourelle 1
    commands.spawn(
        TextBundle::from_section("1 - Tourelle 1 : 30 G", TextStyle {
            font_size: 30.0,
            ..default()
        }).with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(70.0),
            right: Val::Px(10.0),
            ..default()
        })
    );
    // Touche 2 -> Tourelle 2
    commands.spawn(
        TextBundle::from_section("2 - Tourelle 2 : 50 G", TextStyle {
            font_size: 30.0,
            ..default()
        }).with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(130.0),
            right: Val::Px(10.0),
            ..default()
        })
    );
    // Touche P -> Pause
    commands.spawn(
        TextBundle::from_section("P - Pause", TextStyle {
            font_size: 30.0,
            ..default()
        }).with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(240.0),
            right: Val::Px(10.0),
            ..default()
        })
    );
    // Placer une tour
    commands.spawn(
        TextBundle::from_section("Clic gauche", TextStyle {
            font_size: 30.0,
            ..default()
        }).with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(300.0),
            right: Val::Px(10.0),
            ..default()
        })
    );

    commands.spawn(
        TextBundle::from_section("Placer une tour", TextStyle {
            font_size: 30.0,
            ..default()
        }).with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(330.0),
            right: Val::Px(10.0),
            ..default()
        })
    );
    // Détruire une tour
    commands.spawn(
        TextBundle::from_section("Clic droit", TextStyle {
            font_size: 30.0,
            ..default()
        }).with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(400.0),
            right: Val::Px(10.0),
            ..default()
        })
    );

    commands.spawn(
        TextBundle::from_section("Detruire une tour", TextStyle {
            font_size: 30.0,
            ..default()
        }).with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(430.0),
            right: Val::Px(10.0),
            ..default()
        })
    );
}

#[derive(Component)]
pub struct DebugText;

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