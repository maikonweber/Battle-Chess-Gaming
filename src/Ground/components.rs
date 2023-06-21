use bevy::prelude::*;

#[derive(Component)]

pub struct Ground {
    pub ground_type: GroundType,
    pub name: String,
    pub hash: String,
    pub width: u16,
    pub height: u16

}

pub enum GroundType {
    Rock,
    Plant,
    Sand,
    River,
}
