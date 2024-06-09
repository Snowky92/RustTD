use bevy::prelude::*;

#[derive(Resource)]
pub struct Points {
    pub start: (f32, f32),
    pub end: (f32, f32)
}
impl Default for Points {
    fn default() -> Points {
        Points { 
            start: (0.0, 0.0),
            end: (0.0, 0.0)
        }
    }
}