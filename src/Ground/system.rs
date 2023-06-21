use bevy::prelude::{*, shape::Plane};

use super::components::Ground;

pub fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(5.0).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
        Ground {
            name: "RockSmith".to_string(),
            width: 60,
            height: 90,
            hash: "to".to_string(),
            ground_type: super::components::GroundType::Plant,
        },
    ));
}
