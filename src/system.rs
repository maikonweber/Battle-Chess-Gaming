use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::window::PrimaryWindow;


pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

// pub fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>) {
//     for event in game_over_event_reader.iter() {
//         println!("Your final score is: {}", event.score.to_string());
//     }
// }

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera3dBundle {
        projection: OrthographicProjection {
            scale: 3.0,
            scaling_mode: ScalingMode::FixedVertical(2.0),
            ..default()
        }.into(),
        transform: Transform::from_xyz(5.0 , 5.0, 5.0).looking_at(Vec3::ZERO,  Vec3::Y),
        ..default()
    });
}
