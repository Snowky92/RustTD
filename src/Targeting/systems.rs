use std::f32::consts::FRAC_PI_2;

use bevy::{prelude::*, transform::commands, window::PrimaryWindow};

use crate::{DebugText, Enemies::{components::*, ENEMY_SPEED}, Turret::{TURRET_REACH, TURRET_SIZE}, Turrets};
use super::{components::*, BULLET_SIZE, BULLET_SPEED};

/**
 * Regarde si les tourelles sont à portée et désigne leurs cibles
 */
pub fn tracking_target(
    mut commands: Commands,
    enemies_query: Query<&Transform, With<Enemy>>,
    mut turrets_query: Query<(Entity, &Transform, &mut Turrets), With<Turrets>>
) {
    for (i , (turret_entity, turret_transform , mut turret)) in turrets_query.iter_mut().enumerate() {
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
    mut turrets_query: Query<(&mut Transform, &Turrets), (With<Turrets>, With<InRange>)>,
    // mut text_query: Query<&mut Text, With<DebugText>>,
) {
    for (mut turret_transform , turret) in turrets_query.iter_mut() {
        let direction = turret.dir_look - turret_transform.translation;
        let angle = direction.normalize().x.acos(); // Correction : rotation de 90° sinon elle montre le coté

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
    turrets_query: Query<&Transform, (With<Turrets>, With<InRange>)>,
    asset_server: Res<AssetServer>,
) {

    for turret_transform in turrets_query.iter() {

        // TODO : If turret cooldown 0
        
        // Calc vecteur de direction basé sur la rotation
        let theta = 2.0 * turret_transform.rotation.z.atan2(turret_transform.rotation.w);
        let mut direction = Vec3::new( 
            theta.cos(), 
            theta.sin(), 
            0.0
        );
        direction = direction.normalize();

        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(
                        turret_transform.translation.x + (direction.x * (TURRET_SIZE / 2.0)),   // Décalage pour faire sortir la balle du canon
                        turret_transform.translation.y + (direction.y * (TURRET_SIZE / 2.0)), 
                        turret_transform.translation.z + 1.0
                    ),
                    rotation: Quat::from_rotation_z(theta),
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
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut bullets_query: Query<(Entity, &mut Transform, &Bullet), With<Bullet>>,
    time: Res<Time>
) {
    let window = window_query.get_single().unwrap();
    
    for (entity, mut transform, bullet) in bullets_query.iter_mut() {
        let direction = bullet.direction;
        transform.translation += direction * BULLET_SPEED * time.delta_seconds();

        // Despawn les projectiles s'ils vont en dehors de la fenêtre
        if transform.translation.x > window.width() ||
            transform.translation.x < 0.0 ||
            transform.translation.y > window.height() ||
            transform.translation.y < 0.0 {
    
            commands.entity(entity).despawn();
        } 
    }
}
