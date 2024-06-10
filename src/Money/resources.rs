use bevy::prelude::*;

#[derive(Resource)]
pub struct Money {
    pub amount: i32,
}

impl Default for Money {
    fn default() -> Money {
        Money { 
            amount: 60
        }
    }
}