use bevy::prelude::*;

#[derive(Resource)]
pub struct TogglesTurrets {
    pub turret_1: bool,
    pub turret_2: bool
}
impl Default for TogglesTurrets {
    fn default() -> TogglesTurrets {
        TogglesTurrets { 
            turret_1: false,
            turret_2: false,
        }
    }
}


#[derive(Resource)]
pub struct CursorWorldPosition (pub Vec2);
impl Default for CursorWorldPosition {
    fn default() -> Self {
        CursorWorldPosition(Vec2::new(0.0, 0.0))
    }
}

