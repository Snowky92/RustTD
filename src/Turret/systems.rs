use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}, window::PrimaryWindow};

use super::{components::*, TURRET_REACH, TURRET_SIZE};

pub fn handle_right_clicks(
    mut commands: Commands,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    
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
                        transform: Transform::from_xyz(world_position.x, world_position.y, 2.0),
                        texture: asset_server.load("sprites/kenney_tower-defense-top-down/PNG/Default size/towerDefense_tile250.png"),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(TURRET_SIZE, TURRET_SIZE)),
                            ..default()
                        },
                        ..default()
                    },
                    Turrets {
                        dir_look: Vec3::new(0.0, 0.0, 0.0),
                    },
                    Clickable,
                ));
                commands.spawn((
                    MaterialMesh2dBundle {
                        transform: Transform::from_xyz(world_position.x, world_position.y, 1.0),
                        mesh: Mesh2dHandle(meshes.add( Circle { radius: TURRET_REACH })),
                        material: materials.add(Color::rgba(0.0, 0.0, 1.0, 0.1)),
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