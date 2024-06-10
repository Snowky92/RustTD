use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}, window::PrimaryWindow};


use crate::{Map::{components::Tile, TILE_SIZE}, Money::resources::Money, Turret::{BULLET_DAMAGE_F, BULLET_DAMAGE_S, BULLET_SPEED_S, COOLDOWN_S, REACH_S, TURRET_F_COST, TURRET_S_COST}};

use super::{components::*, CursorWorldPosition, TogglesTurrets, BULLET_SPEED_F, COOLDOWN_F, REACH_F, TURRET_SIZE};

pub fn handle_turret_toggle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut toggles: ResMut<TogglesTurrets>
) {
    if keyboard_input.just_pressed(KeyCode::Numpad1) 
    || keyboard_input.just_pressed(KeyCode::Digit1) {
        toggles.turret_1 = !toggles.turret_1;

        toggles.turret_2 = false;
    }

    if keyboard_input.just_pressed(KeyCode::Numpad2) 
    || keyboard_input.just_pressed(KeyCode::Digit2) {
        toggles.turret_2 = !toggles.turret_2;

        toggles.turret_1 = false;
    }
}

pub fn calc_world_coord(
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut cursor_world_pos: ResMut<CursorWorldPosition>,
) {
    let window = windows.get_single().unwrap();
    
    if let Some(mut screen_pos) = window.cursor_position() {
        for (camera, camera_transform) in camera_query.iter() {

            screen_pos.y = window.height() - screen_pos.y; 
          
            let window_size = Vec2::new(window.width() as f32, window.height() as f32);
            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
            let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
            let world_position = ndc_to_world.project_point3(ndc.extend(-1.0)).truncate();
            
            cursor_world_pos.0 = world_position;
        }
    }
}

pub fn handle_left_clicks(
    mut commands: Commands,
    tiles_query: Query<(&Transform, &Tile), With<Tile>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut money: ResMut<Money>,
    asset_server: Res<AssetServer>,
    toggles: Res<TogglesTurrets>,
    cursor_world_pos: Res<CursorWorldPosition>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let world_position = cursor_world_pos.0;

        let mut tile_place:Vec2 = Vec2::ZERO;

        for (transform_tile, tile) in tiles_query.iter() {
            if (world_position.x >= transform_tile.translation.x - (TILE_SIZE / 2.0) && world_position.x < transform_tile.translation.x + (TILE_SIZE / 2.0))
            && (world_position.y >= transform_tile.translation.y - (TILE_SIZE / 2.0) && world_position.y < transform_tile.translation.y + (TILE_SIZE / 2.0)) {
                
                if tile.terrain == 4 {
                    tile_place = Vec2::new(
                        transform_tile.translation.x, 
                        transform_tile.translation.y 
                    );
                }
            }
        }

        if tile_place == Vec2::ZERO {
            return;
        }


        const DEBUG: bool = true;
        if toggles.turret_1 {
            // Tour n1 
            if money.amount >= TURRET_F_COST {
                money.amount -= TURRET_F_COST;
                                
                let turret = commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(tile_place.x, tile_place.y, 2.0),
                        texture: asset_server.load("sprites/kenney_tower-defense-top-down/PNG/Default size/towerDefense_tile249.png"),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(TURRET_SIZE, TURRET_SIZE)),
                            ..default()
                        },
                        ..default()
                    },
                    Turrets {
                        dir_look: Vec3::new(0.0, 0.0, 0.0),
                        b_speed: BULLET_SPEED_F,
                        cooldown: COOLDOWN_F,
                        cooldown_max: COOLDOWN_F,
                        reach: REACH_F,
                        b_damage: BULLET_DAMAGE_F,
                    },
                    Clickable,
                    Tfast
                )).id();

                if DEBUG {
                    let zone = commands.spawn((
                        MaterialMesh2dBundle {
                            transform: Transform::from_xyz(0.0, 0.0, -1.0),
                            mesh: Mesh2dHandle(meshes.add( Circle { radius: REACH_F })),
                            material: materials.add(Color::rgba(0.0, 0.0, 1.0, 0.1)),
                            ..default()
                        },
                        Clickable,
                    )).id();
                    commands.entity(turret).add_child(zone);
                }
            }
        }else if toggles.turret_2 {
            // Tour n2
            if money.amount >= TURRET_S_COST {
                money.amount -= TURRET_S_COST;
            
                let turret = commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(tile_place.x, tile_place.y, 2.0),
                        texture: asset_server.load("sprites/kenney_tower-defense-top-down/PNG/Default size/towerDefense_tile250.png"),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(TURRET_SIZE, TURRET_SIZE)),
                            ..default()
                        },
                        ..default()
                    },
                    Turrets {
                        dir_look: Vec3::new(0.0, 0.0, 0.0),
                        b_speed: BULLET_SPEED_S,
                        cooldown: COOLDOWN_S,
                        cooldown_max: COOLDOWN_S,
                        reach: REACH_S,
                        b_damage: BULLET_DAMAGE_S,
                    },
                    Clickable,
                    Tslow
                )).id();

                if DEBUG {
                    let zone = commands.spawn((
                        MaterialMesh2dBundle {
                            transform: Transform::from_xyz(0.0, 0.0, -1.0),
                            mesh: Mesh2dHandle(meshes.add( Circle { radius: REACH_S })),
                            material: materials.add(Color::rgba(0.0, 0.0, 1.0, 0.1)),
                            ..default()
                        },
                        Clickable,
                    )).id();
                    commands.entity(turret).add_child(zone);
                }
            }
        }
    } 
}


pub fn handle_right_clicks(
    mut commands: Commands,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    query: Query<(Entity, &Transform, &Sprite, Option<&Tslow>, Option<&Tfast>), With<Clickable>>,
    mut money: ResMut<Money>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    cursor_world_pos: Res<CursorWorldPosition>,
) {
    let world_position = cursor_world_pos.0; 
    if mouse_button_input.just_pressed(MouseButton::Right) {  
        for (entity, transform, sprite, tslow, tfast) in query.iter() {
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
                commands.entity(entity).despawn_recursive();
                match (tfast, tslow) {
                    (Some(_), None) => {
                        money.amount += TURRET_F_COST / 2;                        
                    }
                    (None, Some(_)) => {
                        money.amount += TURRET_S_COST / 2; 
                    }
                    _ => {
                            
                    }
                }

            }
        }
    }
}