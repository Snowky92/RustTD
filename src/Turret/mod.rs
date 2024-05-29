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