use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

pub mod components;
mod resources;
mod systems;

use resources::*;
use systems::*;

pub const ENEMY_SPEED: f32 = 50.0; // Enemy speed
pub const ENEMY_SIZE: f32 = 50.0; 

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, spawn_enemies.run_if(on_timer(Duration::from_secs(1))))
            .add_systems(Update, enemy_mov)
            .add_systems(Update, despawn_enemies)
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
    Tire qd à portée 
    Explose ennemis 
        -> Fait perdre de la vie
*/