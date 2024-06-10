use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

pub mod components;
mod resources;
mod systems;

use resources::*;
use systems::*;

pub const ENEMY_SPEED: f32 = 200.0; // Enemy speed
pub const ENEMY_SIZE: f32 = 60.0; 
pub const HEALTH_BAR_SIZE: f32 = 15.0;

pub const ENEMY_PV_1: f32 = 100.0;
pub const ENEMY_SPEED_1: f32 = 100.0;
pub const ENEMY_PV_2: f32 = 200.0;
pub const ENEMY_SPEED_2: f32 = 50.0;

pub const ENEMY_1_MONEY_DROP: i32 = 5;
pub const ENEMY_2_MONEY_DROP: i32 = 10;

pub struct EnemiesPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for EnemiesPlugin<S> {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Difficulty>()
            .add_systems(Startup, detect_count_init)
            .add_systems(Update, spawn_enemies.run_if(on_timer(Duration::from_secs(1))).run_if(in_state(self.state.clone())))
            .add_systems(Update, enemy_mov.run_if(in_state(self.state.clone())))
            // .add_systems(Update, detect_enemy_endzone)
            ;            
    }
}
/*
OBJ :
 X  Spawn Enemies 
 X      -> toutes les 3 secondes
 X  Vont en ligne droite traverser l'écran 
 X  Despawn si les enemis sortent de la fenêtre
 X  Tourelle sur le chemin
 X  Tourelle traque l'ennemi le plus proche
 X  Tire qd à portée 
 X  Explose ennemis 
 X      -> Fait perdre de la vie
*/