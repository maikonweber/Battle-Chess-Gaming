use bevy::prelude::*;

#[derive(Component)]

pub struct Player {
    pub number: u16,
    pub name: String,
    pub hash: String
}
