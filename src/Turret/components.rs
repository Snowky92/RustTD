use std::default;

use bevy::prelude::*;



#[derive(Component)]
pub struct Clickable;

#[derive(Component)]
pub struct Turrets {
    pub dir_look: Vec3,
    pub cooldown: f32,
    pub cooldown_max: f32,
    pub b_speed: f32,
    pub reach: f32
}
#[derive(Component)]
pub struct Tfast;
#[derive(Component)]
pub struct Tslow;
