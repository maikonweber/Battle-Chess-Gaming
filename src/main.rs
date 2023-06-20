use bevy::{prelude::*, transform::commands, window::PrimaryWindow, render::camera::ScalingMode};

fn main () {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camere)
        .add_startup_system(spawn_player)
        .run();

}

#[derive(Component)]
pub struct Player {}
pub fn spawn_player (
   mut commands: Commands,
   window_query: Query<&Window, With<PrimaryWindow>>,
   asset_server: Res<AssetServer> 
) {
    let windown: &Window = window_query.get_single().unwrap();

    commands.spawn( (
        SpriteBundle {
            transform: Transform::from_xyz(windown.width() / 2.0, windown.height() / 2.0, 0.0),
            texture: asset_server.load("/sprites/kenney_mini-dungeon/character-human.png"),
        ..default()
        },
        Player {}
    ));
}


pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn(
        Camera3dBundle {
            projection: OrthographicProjection {
                scale: 3.0,
                scaling_mode: ScalingMode::FixedVertical(2.0),
                ..default()
            }
            .into(), 
            transform : Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y), 
            ..default()
        });   
}
