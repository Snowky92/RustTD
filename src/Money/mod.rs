use bevy::prelude::*;

pub mod resources;
mod systems;

use resources::*;
use systems::*;

pub struct MoneyPlugin;

impl Plugin for MoneyPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Money>()
            .add_systems(Startup, ui_setup)
            .add_systems(Update, update_money_text);
    }
}