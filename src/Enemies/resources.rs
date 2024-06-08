use bevy::prelude::*;

#[derive(Resource)]
pub struct Difficulty {
    pub level: usize,
}
impl Default for Difficulty {
    fn default() -> Difficulty {
        Difficulty { 
            level: 0
        }
    }
}