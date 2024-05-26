use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;

use resources::*;
use systems::*;

pub const ENEMY_SPEED: f32 = 200.0; // Enemy speed
pub const ENEMY_SIZE: f32 = 50.0; 

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, start_spawn_enemies)
            .add_systems(Update, enemy_mov)
            ;            
    }
}