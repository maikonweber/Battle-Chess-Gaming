use bevy::prelude::*;

mod Player;
use Player::PlayerPlugin;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .run();
}






// #[derive(Component)]
// pub struct Player {}

// pub fn spawn_player(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     window_query: Query<&Window, With<PrimaryWindow>>,
//     asset_server: Res<AssetServer>,
// ) {
//     let windown = window_query.get_single().unwrap();

//     commands.spawn((
//         PbrBundle {
//             mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
//             material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
//             transform: Transform::from_xyz(1.5, 0.5, 1.5),
//             ..default()
//         },
//         Player {},
//     ));
// }

// pub fn player_movement(
//     keyboard_input: Res<Input<KeyCode>>,
//     mut player_query: Query<&mut Transform, With<Player>>,
//     time: Res<Time>,
// ) {
//     if let Ok(mut transform) = player_query.get_single_mut() {
//         let mut direction = Vec3::ZERO;

//         if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
//             direction += Vec3::new(0.4, 0.0, 0.0);
//         }
//         if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
//             direction += Vec3::new(-0.4, 0.0, 0.0);
//         }
//         if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
//             direction += Vec3::new(0.0, 0.4, 0.0);
//         }
//         if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
//             direction += Vec3::new(0.0, -0.4, 0.0);
//         }

//         if direction.length() > 0.0 {
//             direction = direction.normalize();
//         }

//         transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
//     }
// }

// fn setup(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     // camera
//     commands.spawn(Camera3dBundle {
//         projection: OrthographicProjection {
//             scale: 3.0,
//             scaling_mode: ScalingMode::FixedVertical(2.0),
//             ..default()
//         }
//         .into(),
//         transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
//         ..default()
//     });

//     // plane
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(shape::Plane::from_size(5.0).into()),
//         material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
//         ..default()
//     });
//     // cubes

//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
//         material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
//         transform: Transform::from_xyz(1.5, 0.5, -1.5),
//         ..default()
//     });
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
//         material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
//         transform: Transform::from_xyz(-1.5, 0.5, 1.5),
//         ..default()
//     });
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
//         material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
//         transform: Transform::from_xyz(-1.5, 0.5, -1.5),
//         ..default()
//     });
//     // light
//     commands.spawn(PointLightBundle {
//         transform: Transform::from_xyz(3.0, 8.0, 5.0),
//         ..default()
//     });
// }
