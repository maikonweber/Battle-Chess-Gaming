use std::collections::hash_map;
use std::hash::Hash;

use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::{prelude::*, animation};
use bevy::utils::HashMap;
use bevy::window::PrimaryWindow;

use super::components::Player;


pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/character-orc.png"),
            ..default()
        },
        Player {
            number: 1,
            name: "Maikon".to_string(),
            hash: "Hash".to_string(),
        },
    ));
}

pub fn print_mouse_event_system(
    mut mouse_button_input_events: EventReader<bevy::input::mouse::MouseButtonInput>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_wheel_events: EventReader<MouseWheel>
) {
    for event in mouse_button_input_events.iter() {
        info!("{:?}", event);
    } for event in mouse_motion_events.iter() {
        info!("{:?}", event);
    }
     for event in cursor_moved_events.iter() {
        info!("{:?}", event);
    }

     for event in mouse_wheel_events.iter() {
        info!("{:?}", event);
    }



}



// pub fn create_player_anim_hashmap() -> HashMap<String, animation::AnimationPlayer>
// {
//     let mut hash_map:  = HashMap::new();

//     hash_map.insert("Walk".to_string(), animation::Animation(start:1, end: 3, looping: true, cooldown: 0.1));
//     hash_map.insert("idle".to_string(), animation::Animation(start:1, end: 1, looping: true, cooldown: 0.1));
    
//     return hash_map
// }


pub fn debugger(player_query: Query<&Player>) {
    for person in player_query.iter() {
        println!("Player {}", person.name);
    }
}
