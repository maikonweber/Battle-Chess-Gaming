pub const numRows: u32 = 30;
pub const numColums: u32 = 30;

use super::components::Ground;
use bevy::prelude::{shape::{Plane, RegularPolygon}, *};


#[derive(Component)]
struct Component1;

pub fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for row in 0..numRows {
        for column in 0..numColums {
            let position = Vec3::new(column as f32, 0.0, row as f32);
            println!("{} ",position.to_string());
        }
    }
}
