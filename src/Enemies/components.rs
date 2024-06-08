use bevy::prelude::*;


#[derive(Component)]
pub struct Enemy {
    pub e_type: u32,
    pub direction: Vec2,
    pub pv: f32
}