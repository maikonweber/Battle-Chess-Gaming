use bevy::log::LogSettings;
use local_ip_address::local_ip;
use logic_renet_demo::*;
use std::{
net::{SocketAddr, UdpSocket}
};
use std::time::SystemTime;
use dbg::dbg;

pub fn create_renet_client() -> RenetClient {
    let current_time = SystemTime::now();
    dbg!(current_time);

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    
    let client_id = current_time.as_millis() as u64;

    let server_addr = SocketAddr::new(local_ip().unwrap(), 42069);


    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id : PROTOCOL_ID,
        server_addr,
        user_data: None
    };

    RenetPlugin::new {
        current_time, socket, client_id, connection_config, 
        authentication
    }
    .unwrap()
}


pub fn client_ping(mut client: ResMut<RenetClient>, keyboard: Res<Input<KeyCode>>) {
    let reliable_channel_id = ReliableChannelConfig::default().channel_id;

    if keyboard.just_pressed(KeyCode::Space) {
        let ping_message = bincode::serialize(&ClientMessage::Ping).unwrap();
        client.send_message(reliable_channel_id, ping_message);
        info!("Sent ping!");
    }

    while let Some(message) = client.receive_message(reliable_channel_id) {
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessage::Pong => {
                info!("Got pong!");
            }
        }
    }
}