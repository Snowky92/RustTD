#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(private_interfaces)]

use std::process::Command;

use bevy::log::tracing_subscriber::filter;
use bevy::window::{PrimaryWindow, WindowMode, WindowResolution};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::{prelude::*, transform, window};
use rand::random;

use Map::MapPlugin;
mod Map;
use Turret::TurretPlugin;
mod Turret;
use Enemies::EnemiesPlugin;
use Targeting::TargetingPlugin;

mod Enemies;
mod Targeting;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "RustTD".into(),
                name: Some("bevy.app".into()),
                resizable: false,
                mode: WindowMode::Windowed,
                resolution: WindowResolution::new(1920.0, 1080.0).with_scale_factor_override(1.0),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(TurretPlugin)
        .add_plugins(EnemiesPlugin)
        .add_plugins(TargetingPlugin)
        .add_systems(Startup, spawn_camera)
        .add_plugins(MapPlugin)
        .run();
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub const TURRET_SIZE: f32 = 50.0;
pub const TURRET_REACH: f32 = 200.0;

#[derive(Component)]
pub struct Turrets {
    pub dir_look: Vec3,
}

pub fn spawn_test_turret(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = window_query.get_single().unwrap();

    let turret_x = window.width() / 4.0 ;
    let turret_y = (window.height() / 2.0) + 100.0;

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(turret_x, turret_y, 2.0),
            texture: asset_server.load("sprites/kenney_tower-defense-top-down/PNG/Default size/towerDefense_tile250.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(TURRET_SIZE, TURRET_SIZE)),
                ..default()
            },
            ..default()
        },
        Turrets {
            dir_look: Vec3::new(0.0, 0.0, 0.0),
        }
    ));

    commands.spawn(MaterialMesh2dBundle {
        transform: Transform::from_xyz(turret_x, turret_y, 0.0),
        mesh: Mesh2dHandle(meshes.add( Circle { radius: TURRET_REACH })),
        material: materials.add(Color::rgba(0.0, 0.0, 1.0, 0.1)),
        ..default()
    });
}
