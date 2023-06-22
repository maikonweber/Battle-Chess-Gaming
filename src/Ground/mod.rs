use bevy::prelude::*;

pub mod components;
mod system;

use system::*;
pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_ground);
         
    }
}