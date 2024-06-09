use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::Enemies::ENEMY_SPEED;

pub struct TurretPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for TurretPlugin<S> {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<TogglesTurrets>()
            .add_systems(Update, handle_right_clicks.run_if(in_state(self.state.clone())))
            .add_systems(Update, handle_left_clicks.run_if(in_state(self.state.clone())))
            .add_systems(Update, handle_turret_toggle.run_if(in_state(self.state.clone())))
            ;            
    }
}

pub const TURRET_SIZE: f32 = 50.0;

pub const BULLET_SPEED_F: f32 = ENEMY_SPEED * 10.0;
pub const REACH_F: f32 = 200.0;
pub const BULLET_DAMAGE_F: f32 = 20.0;
pub const COOLDOWN_F: f32 = 0.2;

pub const BULLET_SPEED_S: f32 = ENEMY_SPEED * 10.0;
pub const REACH_S: f32 = 300.0;
pub const BULLET_DAMAGE_S: f32 = 80.0;
pub const COOLDOWN_S: f32 = 1.0;