#![allow(non_snake_case)] 
#![allow(unused_imports)]

use std::process::Command;

use bevy::{prelude::*, transform, window};
use bevy::window::PrimaryWindow;
use rand::random;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}

