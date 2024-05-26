use bevy::prelude::*;
use bevy::render::view::window;
use bevy::window::PrimaryWindow;

use super::{ENEMY_SIZE, ENEMY_SPEED}; 
use super::components::*;

pub fn start_spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(50.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/kenney_tower-defense-top-down/PNG/Default size/towerDefense_tile270.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(ENEMY_SIZE, ENEMY_SIZE)),
                ..default()
            },
            ..default()
        },
        Enemy {
            e_type: 0,
            direction: Vec2::new(1.0, 0.0),
        }
    ));
}

pub fn enemy_mov (
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>
) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}