use bevy::prelude::*;

#[derive(Component)]
pub struct InRange;

#[derive(Component)]
pub struct Bullet {
    pub direction: Vec3
}
