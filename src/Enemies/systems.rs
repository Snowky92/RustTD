use bevy::prelude::*;
use bevy::render::view::window;
use bevy::window::PrimaryWindow;
use rand::random;

use super::{Difficulty, ENEMY_PV_1, ENEMY_PV_2, ENEMY_SIZE, ENEMY_SPEED, ENEMY_SPEED_1, ENEMY_SPEED_2}; 
use super::components::*;


/**
 * Fait spawn des ennemis
 */
pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut difficulty: ResMut<Difficulty>, 
) {
    let window = window_query.get_single().unwrap();

    let random = random::<f32>() * 100.0; // Random 0 à 100

    if random > difficulty.level as f32 {
        // Spawn ennemi n1
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(50.0, window.height() / 2.0, 1.0),
                texture: asset_server.load("sprites/kenney_tower-defense-top-down/PNG/Default size/towerDefense_tile245.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(ENEMY_SIZE, ENEMY_SIZE)),
                    ..default()
                },
                ..default()
            },
            Enemy {
                direction: Vec2::new(1.0, 0.0),
                pv: ENEMY_PV_1,
                speed: ENEMY_SPEED_1,
            }
        ));
    } else {
        // Spawn ennemi n2
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(50.0, window.height() / 2.0, 1.0),
                texture: asset_server.load("sprites/kenney_tower-defense-top-down/PNG/Default size/towerDefense_tile246.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(ENEMY_SIZE, ENEMY_SIZE)),
                    ..default()
                },
                ..default()
            },
            Enemy {
                direction: Vec2::new(1.0, 0.0),
                pv: ENEMY_PV_2,
                speed: ENEMY_SPEED_2,
            }
        ));
    }

    difficulty.level += 1;
}

/**
 * Déplace les ennemis à chaque frame dans la direction où ils vont
 */
pub fn enemy_mov (
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut enemy_query: Query<(Entity, &mut Transform, &Enemy)>,
    time: Res<Time>
) {
    let window = window_query.get_single().unwrap();

    for (entity, mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * enemy.speed * time.delta_seconds();

        // Despawn les ennemies s'ils vont en dehors de la fenêtre
        if transform.translation.x > window.width() ||
            transform.translation.x < 0.0 ||
            transform.translation.y > window.height() ||
            transform.translation.y < 0.0 {
    
            commands.entity(entity).despawn();
        }  
    }
}