#![allow(non_snake_case)] 
#![allow(unused_imports)]
#![allow(private_interfaces)]

use std::process::Command;

use bevy::{prelude::*, transform, window};
use bevy::window::PrimaryWindow;
use rand::random;


use Turret::TurretPlugin;
mod Turret;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TurretPlugin)
        .add_systems(Startup, spawn_camera)
        .run();
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        }
    );
}