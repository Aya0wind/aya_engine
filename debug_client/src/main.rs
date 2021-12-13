use std::net::SocketAddr;
use std::time::Duration;

use bevy::prelude::*;
use bevy_spicy_networking::{ClientNetworkEvent, NetworkClient, NetworkData, NetworkServer, NetworkSettings, ServerNetworkEvent};

use crate::shared::{SendToServerMessage, SendToUserMessage};

mod shared;

fn main() {
    let mut app = App::build();

    app.add_plugins(DefaultPlugins);

    // You need to add the `ClientPlugin` first before you can register
    // `ClientMessage`s
    app.add_plugin(bevy_spicy_networking::ClientPlugin);

    // A good way to ensure that you are not forgetting to register
    // any messages is to register them where they are defined!
    shared::client_register_network_messages(&mut app);
    app.add_startup_system(connect_start_system.system());
    app.add_system(handle_incoming_messages.system());
    app.insert_resource(MyTimer(Timer::new(Duration::from_secs(3), true)));
    app.run();
}

struct MyTimer(Timer);

fn frequently_send_message(mut net: Res<NetworkServer>, time: Res<Time>, mut timer: ResMut<MyTimer>) {
    if timer.0.tick(time.delta()).just_finished() {
        net.broadcast(SendToUserMessage { message: "message".to_string() })
    }
}


fn connect_start_system(
    mut net: ResMut<NetworkClient>,
) {
    let ip_address = "127.0.0.1".parse().unwrap();

    info!("Address of the server: {}", ip_address);

    let socket_address = SocketAddr::new(ip_address, 9999);

    net.connect(
        socket_address,
        NetworkSettings {
            max_packet_length: 10 * 1024 * 1024,
        },
    );
}

fn handle_incoming_messages(mut new_messages: EventReader<NetworkData<shared::SendToServerMessage>>) {
    for new_message in new_messages.iter() {
        eprintln!("{:?}", new_message);
    }
}

fn handle_connection_events(
    net: Res<NetworkServer>,
    mut network_events: EventReader<ServerNetworkEvent>,
) {
    for event in network_events.iter() {
        match event {
            &ServerNetworkEvent::Connected(conn_id) => {
                net.send_message(conn_id, SendToUserMessage { message: "message".to_string() });
                info!("New client connected: {:?}", conn_id);
            }
            _ => (),
        }
    }
}