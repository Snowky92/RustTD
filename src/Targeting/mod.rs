use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use systems::*;

use crate::{Enemies::ENEMY_SPEED, Turret::TURRET_SIZE};

pub mod components;
mod systems;

pub const BULLET_SIZE: f32 = TURRET_SIZE / 4.0 * 3.0;
pub const BULLET_SPEED: f32 = ENEMY_SPEED * 10.0;

pub struct TargetingPlugin;

impl Plugin for TargetingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (tracking_target, mov_turret).chain())
            .add_systems(Update, shoot.run_if(on_timer(Duration::from_millis(700))))
            .add_systems(Update, mov_bullets)
            .add_systems(Update, kill_enemies)
            ;            
    }
}