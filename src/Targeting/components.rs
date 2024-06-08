use bevy::prelude::*;

#[derive(Component)]
pub struct InRange;

#[derive(Component)]
pub struct Bullet {
    pub direction: Vec3, 
    pub damage: f32,
    pub speed: f32
}

#[derive(Component)]
pub struct InCooldown;

