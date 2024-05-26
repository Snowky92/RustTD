#![allow(non_snake_case)] 
#![allow(unused_imports)]

use std::process::Command;

use bevy::{prelude::*, transform, window};
use bevy::window::PrimaryWindow;
use rand::random;
use Enemies::EnemiesPlugin;

mod Enemies;
pub const MAX_ENEMIES: f32 = 10.0;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EnemiesPlugin)
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