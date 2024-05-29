#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(private_interfaces)]

use std::process::Command;

use bevy::log::tracing_subscriber::filter;
use bevy::window::{PrimaryWindow, WindowMode, WindowResolution};
use bevy::{prelude::*, transform, window};
use rand::random;

use Map::MapPlugin;
mod Map;
use Turret::TurretPlugin;
mod Turret;


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
