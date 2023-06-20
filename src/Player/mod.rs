use bevy::prelude::*;

pub mod components;
mod system;

use system::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
    }
}