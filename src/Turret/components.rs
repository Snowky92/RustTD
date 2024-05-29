use bevy::prelude::*;



#[derive(Component)]
pub struct Clickable;

#[derive(Component)]
pub struct Turrets {
    pub dir_look: Vec3,
}