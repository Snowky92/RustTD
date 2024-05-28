use bevy::prelude::*;
use systems::*;

mod systems;

pub struct TargetingPlugin;

impl Plugin for TargetingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (tracking_target, mov_turret).chain())
            ;            
    }
}