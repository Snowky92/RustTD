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