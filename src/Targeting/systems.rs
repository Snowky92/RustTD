use std::{f32::consts::FRAC_PI_2, process::Child};

use bevy::{ecs::entity, prelude::*, transform::{self, commands}, window::PrimaryWindow};

use crate::{DebugText, Enemies::{components::*, ENEMY_SIZE, ENEMY_SPEED}, Turret::{self, components::Turrets, BULLET_DAMAGE_F, BULLET_SPEED_F, REACH_F, TURRET_SIZE}};
use super::{components::*, BULLET_SIZE};

/**
 * Regarde si les tourelles sont à portée et désigne leurs cibles
 */
pub fn tracking_target(
    mut commands: Commands,
    enemies_query: Query<&Transform, With<Enemy>>,
    mut turrets_query: Query<(Entity, &Transform, &mut Turrets), With<Turrets>>
) {
    for (turret_entity, turret_transform , mut turret) in turrets_query.iter_mut() {
        let mut direction: Vec3 = Vec3::ZERO;
        let mut closer: f32 = REACH_F;
        let mut in_range = false;
        
        for enemy_transform in enemies_query.iter() {
            let distance = enemy_transform.translation.distance(turret_transform.translation);

            if closer > distance {
                // Si l'ennemi est à portée de la tourelle, et est le plus proche
                closer = distance;
                direction = enemy_transform.translation;
                in_range = true;
            }
        }

        if in_range && closer < REACH_F {
            // S'il y avait un ennemi à portée
            turret.dir_look = direction;

            commands.entity(turret_entity).insert(InRange);
        } else {
            commands.entity(turret_entity).remove::<InRange>();
        }
    }
}

/**
 * Fait tourner les tourelles à portée de tir
 */
pub fn mov_turret(
    mut turrets_query: Query<(&mut Transform, &Turrets), (With<Turrets>, With<InRange>)>,
    // mut text_query: Query<&mut Text, With<DebugText>>,
) {
    for (mut turret_transform , turret) in turrets_query.iter_mut() {
        let direction = turret.dir_look - turret_transform.translation;
        let angle = direction.y.atan2(direction.x);

        turret_transform.rotation = Quat::from_rotation_z(angle);

        // let mut text = text_query.get_single_mut().unwrap();
        // text.sections[0].value = format!("{:.2}", turret_transform.rotation.z.to_degrees());
    }
}

/**
 * Fait tirer les tourelles à portée de tir qui sont au bout de leur cooldown
 */
pub fn shoot (
    mut commands: Commands,
    turrets_query: Query<(&Transform, Entity, &Turrets), (With<Turrets>, With<InRange>, Without<InCooldown>)>,
    asset_server: Res<AssetServer>,
) {

    for (turret_transform, turret_entity, turret) in turrets_query.iter() {

        // Calc vecteur de direction basé sur la rotation
        // let theta = 2.0 * turret_transform.rotation.z.atan2(turret_transform.rotation.w);
        let angle_z = turret_transform.rotation.to_euler(EulerRot::ZXY).0;
        let mut direction = Vec3::new( 
            angle_z.cos(), 
            angle_z.sin(), 
            0.0
        );
        direction = direction.normalize();

        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(
                        turret_transform.translation.x + (direction.x * (TURRET_SIZE / 3.0)),   // Décalage pour faire sortir la balle du canon
                        turret_transform.translation.y + (direction.y * (TURRET_SIZE / 3.0)), 
                        turret_transform.translation.z + 1.0
                    ),
                    rotation: Quat::from_rotation_z(angle_z),
                    ..default()
                },
                texture: asset_server.load("sprites/kenney_tower-defense-top-down/PNG/Default size/towerDefense_tile251.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(BULLET_SIZE, BULLET_SIZE)),
                    ..default()
                },
                ..default()
            },
            Bullet {
                direction: direction,
                speed: turret.b_speed,
                damage: turret.b_damage
            }
        ));

        commands.entity(turret_entity).insert(InCooldown);
    }
}

/**
 * Déplace toutes les balles vers leur direction
 */
pub fn mov_bullets (
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut bullets_query: Query<(Entity, &mut Transform, &Bullet), With<Bullet>>,
    time: Res<Time>
) {
    let window = window_query.get_single().unwrap();
    
    for (entity, mut transform, bullet) in bullets_query.iter_mut() {
        let direction = bullet.direction;
        transform.translation += direction * BULLET_SPEED_F * time.delta_seconds();

        // Despawn les projectiles s'ils vont en dehors de la fenêtre
        if transform.translation.x > window.width() ||
            transform.translation.x < 0.0 ||
            transform.translation.y > window.height() ||
            transform.translation.y < 0.0 {
    
            commands.entity(entity).despawn();
        } 
    }
}

/**
 * Détecte la collision entre les balles et les ennemis
 */
pub fn hit_enemies (
    mut commands: Commands,
    bullets_query: Query<(Entity, &Transform, &Bullet), With<Bullet>>,
    mut enemy_query: Query<(Entity, &Transform, &mut Enemy, &Children), With<Enemy>>,
) {
    let mut hits: Vec<(Entity, Entity, f32, Vec<Entity>)> = Vec::new();

    for (bullet_entity, bullet_transform, bullet) in bullets_query.iter() {
        for (enemy_entity, &enemy_transform, mut enemy, children) in enemy_query.iter_mut() {

            let distance = bullet_transform.translation.distance(enemy_transform.translation);

            let bullet_radius = BULLET_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;

            // Si la distance est inférieur à leur 2 rayons, ils se touchent
            if distance < bullet_radius + enemy_radius {
                // Touché !
                enemy.pv -= bullet.damage;
                commands.entity(bullet_entity).despawn();
                
                if enemy.pv <= 0.0 {
                    // Boum
                    commands.entity(enemy_entity).despawn_recursive();
                }
                break;
            }
        }
    }
}

pub fn update_health_bar(
    mut health_bar_query: Query<(&Parent, &mut Transform, &HealthBar), With<HealthBar>>,
    parent_query: Query<&Enemy, With<Enemy>>
) {
    for (parent, mut health_transform, health_bar) in health_bar_query.iter_mut() {
        let entity = parent.get();
        let Ok(enemy) = parent_query.get(entity) else {
            continue;
        };

        let percent = enemy.pv / health_bar.max;

        health_transform.scale.x = percent;
    }
}


/**
 * Gère le cooldown des tourelles, le fait réduire et retire le tag s'il est arrivé au bout
 */
pub fn handle_cooldown(
    mut commands: Commands,
    mut turrets_query: Query<(&mut Turrets, Entity), (With<Turrets>, With<InCooldown>)>,
    time: Res<Time>,
) {
    for (mut turret, turret_entity) in turrets_query.iter_mut() {
        if turret.cooldown <= 0.0 {
            commands.entity(turret_entity).remove::<InCooldown>();
            turret.cooldown = turret.cooldown_max;
        } else {
            turret.cooldown -= time.delta_seconds();
        }
    }
}