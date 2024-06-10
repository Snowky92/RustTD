use bevy::prelude::*;
use bevy::render::view::window;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;
use rand::random;

use super::{Difficulty, ENEMY_PV_1, ENEMY_PV_2, ENEMY_SIZE, ENEMY_SPEED, ENEMY_SPEED_1, ENEMY_SPEED_2, HEALTH_BAR_SIZE}; 
use crate::GameState;
use crate::Map::components::Points;

use super::components::*;


pub fn detect_count_init(
    mut commands: Commands
) {
    commands.spawn(DetectCount {
        maxCountBeforeDeath: 3,
        currentCount: 0
    });
}

/**
 * Fait spawn des ennemis
 */
pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut difficulty: ResMut<Difficulty>, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = window_query.get_single().unwrap();

    let random = random::<f32>() * 100.0; // Random 0 à 100

    let enemy_entity;

    let mut max_pv: f32 = 0.0;

    if random > difficulty.level as f32 {
        // Spawn ennemi n1
        enemy_entity = commands.spawn((
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
                enemy_type: 1
            }
        )).id();
        max_pv = ENEMY_PV_1;
    } else {
        // Spawn ennemi n2
        enemy_entity = commands.spawn((
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
                enemy_type: 2
            }
        )).id();
        max_pv = ENEMY_PV_2;
    }

    let health_bar_entity = commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Rectangle::new(HEALTH_BAR_SIZE, 2.0))
                .into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_translation(Vec3::new(0.0, -(ENEMY_SIZE / 3.0), 1.0)),
            ..default()
        },
        HealthBar {
            max: max_pv
        }
    )).id();

    commands.entity(enemy_entity).add_child(health_bar_entity);

    difficulty.level += 1;
}

/**
 * Permet de supprimer l'ennemi qui a atteint la case de fin
 * Cela retire un point de vie si c'est le cas
 */
pub fn detect_enemy_endzone(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    mut detectCount_query: Query<&mut DetectCount>,
    points: Query<&Points>
) {
    let mut counter = detectCount_query.get_single_mut().unwrap();

    for (enemy_entity, transform) in enemy_query.iter() {
        if transform.translation.x >= points.single().end.0 && transform.translation.y >= points.single().end.1 {
            commands.entity(enemy_entity).despawn();

            counter.currentCount += 1;

            if counter.currentCount == counter.maxCountBeforeDeath {
                next_state.set(GameState::Paused);
                commands.spawn(
                    
                    TextBundle::from_section(
                        
                        "Vous avez perdu !",
                        TextStyle {
                            font_size: 100.0,
                            ..default()
                        },
                    )
                    .with_text_justify(JustifyText::Center)
                    
                    .with_style(Style {
                        width: Val::Percent(100.),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    })
                );
            }
        }
    }
}

/**
 * Despawn les ennemies s'ils vont en dehors de la fenêtre
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
    
            commands.entity(entity).despawn_recursive();
        }  
    }
}