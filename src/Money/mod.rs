use bevy::prelude::*;

pub mod resources;

use resources::*;

pub struct MoneyPlugin;

impl Plugin for MoneyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Money>();
    }
}