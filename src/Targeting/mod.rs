use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use systems::*;

use crate::{Enemies::ENEMY_SPEED, Turret::TURRET_SIZE};

pub mod components;
mod systems;

pub const BULLET_SIZE: f32 = TURRET_SIZE / 4.0 * 3.0;

pub struct TargetingPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for TargetingPlugin<S> {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (tracking_target.run_if(in_state(self.state.clone())), mov_turret.run_if(in_state(self.state.clone()))).chain())
            .add_systems(Update, shoot.run_if(in_state(self.state.clone())))
            .add_systems(Update, handle_cooldown.run_if(in_state(self.state.clone())))
            .add_systems(Update, mov_bullets.run_if(in_state(self.state.clone())))
            .add_systems(Update, hit_enemies.run_if(in_state(self.state.clone())))
            .add_systems(Update, update_health_bar.run_if(in_state(self.state.clone())))
            ;            
    }
}