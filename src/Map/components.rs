use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize, Debug)]
pub struct Map {
    pub cases: Vec<Vec<i32>>
}