use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;

use resources::*;
use systems::*;

pub struct TurretPlugin;

impl Plugin for TurretPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, handle_right_clicks)
            .add_systems(Update, handle_left_clicks)
            ;            
    }
}

pub const TURRET_SIZE: f32 = 50.0;
pub const TURRET_REACH: f32 = 200.0;