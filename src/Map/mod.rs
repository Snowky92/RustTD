use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use resources::*;
use systems::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Points>()
            .add_systems(Startup, load_map);
    }
}

pub const TILE_SIZE: f32 = 64.0;