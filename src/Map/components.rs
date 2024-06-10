use bevy::{prelude::*, reflect::Tuple};
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize, Debug)]
pub struct Map {
    pub cases: Vec<Vec<i32>>
}

#[derive(Component)]
pub struct Tile;