use bevy::prelude::*;


#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
    pub pv: f32,
    pub speed: f32,
}

#[derive(Component)]
pub struct HealthBar {
    pub max: f32
}