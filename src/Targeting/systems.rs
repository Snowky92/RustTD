use std::f32::consts::FRAC_PI_2;

use bevy::{prelude::*, transform::commands};

use crate::{Enemies::{components::*, ENEMY_SPEED}, Turret::{TURRET_REACH, TURRET_SIZE}, Turrets};
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
        let mut closer: f32 = TURRET_REACH;
        let mut in_range = false;

        for enemy_transform in enemies_query.iter() {
            let distance = enemy_transform.translation.distance(turret_transform.translation);

            if closer > distance {
                closer = distance;
                direction = enemy_transform.translation;
                in_range = true;
            }

        }

        if in_range && closer < TURRET_REACH {
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
    mut turrets_query: Query<(&mut Transform, &Turrets), (With<Turrets>, With<InRange>)>
) {
    for (mut turret_transform , turret) in turrets_query.iter_mut() {
        let direction = turret.dir_look - turret_transform.translation;
        let angle = direction.y.atan2(direction.x) - FRAC_PI_2; // Correction : rotation de 90° sinon elle montre le coté

        turret_transform.rotation = Quat::from_rotation_z(angle);
    }
}

/**
 * Fait tirer les tourelles à portée de tir qui sont au bout de leur cooldown
 */
pub fn shoot (
    mut commands: Commands,
    turrets_query: Query<&Transform, (With<Turrets>, With<InRange>)>,
    asset_server: Res<AssetServer>,
) {

    for turret_transform in turrets_query.iter() {

        // TODO : If turret cooldown 0
        
        // Calc vecteur de direction basé sur la rotation
        let direction = Vec3::new(
            f32::cos(turret_transform.rotation.z - FRAC_PI_2) * 1.0, 
            f32::sin(turret_transform.rotation.z - FRAC_PI_2) * 1.0, 
            0.0
        );

        println!("{:?}{:?}{:?}", turret_transform.rotation.z, direction.x, direction.y); 

        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(
                        turret_transform.translation.x + (direction.x * (TURRET_SIZE / 2.0)),   // Décalage pour faire sortir la balle du canon
                        turret_transform.translation.y + (direction.y * (TURRET_SIZE / 2.0)), 
                        turret_transform.translation.z + 1.0
                    ),
                    rotation: Quat::from_rotation_z(turret_transform.rotation.z ),
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
                direction: direction
            }
        ));
    }
}

/**
 * Déplace toutes les balles vers leur direction
 */
pub fn mov_bullets (
    mut bullets_query: Query<(&mut Transform, &Bullet), With<Bullet>>,
    time: Res<Time>
) {
    for (mut transform, bullet) in bullets_query.iter_mut() {
        let direction = bullet.direction;
        transform.translation += direction * (ENEMY_SPEED * 1.0) * time.delta_seconds();
    }
}
