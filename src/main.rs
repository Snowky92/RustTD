#![allow(non_snake_case)] 
#![allow(unused_imports)]

use std::process::Command;

use bevy::{prelude::*, transform, window};
use bevy::window::PrimaryWindow;
use rand::random;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
//        .add_systems(Startup, spawn_grid)
        .add_systems(Update, handle_left_clicks)
        .add_systems(Update, handle_right_clicks)
        .run();
}

#[derive(Component)]
struct Clickable;

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        }
    );
}

pub fn handle_right_clicks(
    mut commands: Commands,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let window = windows.get_single().unwrap();
        if let Some(cursor_position) = window.cursor_position() {
            for (camera, camera_transform) in q_camera.iter() {
                let window_size = Vec2::new(window.width() as f32, window.height() as f32);
                let ndc = (cursor_position / window_size) * 2.0 - Vec2::ONE;
                let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
                let world_position = ndc_to_world.project_point3(ndc.extend(-1.0)).truncate();

                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(0.0, 0.0, 1.0),
                            custom_size: Some(Vec2::new(30.0, 30.0)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(world_position.x, world_position.y, 0.0)),
                        ..default()
                    },
                    Clickable,
                ));
            }
        }
    }
}

pub fn handle_left_clicks(
    mut commands: Commands,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    query: Query<(Entity, &Transform, &Sprite), With<Clickable>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    if mouse_button_input.just_pressed(MouseButton::Right) {
        let window = windows.get_single().unwrap();
        if let Some(cursor_position) = window.cursor_position() {
            for (camera, camera_transform) in q_camera.iter() {
                let window_size = Vec2::new(window.width() as f32, window.height() as f32);
                let ndc = (cursor_position / window_size) * 2.0 - Vec2::ONE;
                
                let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
                let world_position = ndc_to_world.project_point3(ndc.extend(-1.0)).truncate();
                
                for (entity, transform, sprite) in query.iter() {
                    let sprite_pos = transform.translation;
                    let sprite_size = sprite.custom_size.unwrap();
                    let half_size = sprite_size / 2.0;
                    
                    let min_bounds = sprite_pos - Vec3::new(half_size.x, half_size.y, 0.0);
                    let max_bounds = sprite_pos + Vec3::new(half_size.x, half_size.y, 0.0);
                    
                    if world_position.x > min_bounds.x
                    && world_position.x < max_bounds.x
                    && world_position.y > min_bounds.y
                    && world_position.y < max_bounds.y
                    {
                        commands.entity(entity).despawn();
                    }
                }
            }
        }
    }
}
/*
pub fn spawn_grid(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();
    let cell_size = 50.0;

    for y in (0..(window.height() as i32)).step_by(cell_size as usize) {
        for x in (0..(window.width() as i32)).step_by(cell_size as usize) {
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0.9, 0.9, 0.9, 0.5),
                    custom_size: Some(Vec2::new(cell_size - 1.0, cell_size - 1.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(x as f32, y as f32, 0.0)),
                ..default()
            });
        }
    }
}
*/