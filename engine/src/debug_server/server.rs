use bevy::app::ScheduleRunnerSettings;
use bevy::prelude::*;
use bevy_spicy_networking::{ConnectionId, NetworkData, NetworkServer, ServerNetworkEvent};
use std::net::SocketAddr;
use std::time::Duration;

use crate::debug_server::shared;

use super::shared::server_register_network_messages;

pub struct DebugServer;


impl Plugin for DebugServer {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(bevy_spicy_networking::ServerPlugin);

        // A good way to ensure that you are not forgetting to register
        // any messages is to register them where they are defined!
        server_register_network_messages(app);

        app.add_startup_system(setup_networking.system());
        app.add_system(handle_connection_events.system());
        app.add_system(handle_messages.system());
    }
}

// On the server side, you need to setup networking. You do not need to do so at startup, and can start listening
// at any time.
fn setup_networking(mut net: ResMut<NetworkServer>) {
    let ip_address = "127.0.0.1".parse().expect("Could not parse ip address");

    println!("Address of the server: {}", ip_address);

    let socket_address = SocketAddr::new(ip_address, 9999);

    match net.listen(socket_address) {
        Ok(_) => (),
        Err(err) => {
            error!("Could not start listening: {}", err);
            panic!();
        }
    }

    println!("Started listening for new connections!");
}

struct Player(ConnectionId);

fn handle_connection_events(
    mut commands: Commands,
    net: Res<NetworkServer>,
    mut network_events: EventReader<ServerNetworkEvent>,
) {
    for event in network_events.iter() {
        if let ServerNetworkEvent::Connected(conn_id) = event {
            commands.spawn_bundle((Player(*conn_id), ));

            // Broadcasting sends the message to all connected players! (Including the just connected one in this case)
            net.broadcast(shared::NewChatMessage {
                name: String::from("SERVER"),
                message: format!("New user connected; {:?}", conn_id),
            });
            println!("New player connected: {:?}", conn_id);
        }
    }
}

// Receiving a new message is as simple as listening for events of `NetworkData<T>`
fn handle_messages(
    mut new_messages: EventReader<NetworkData<shared::SendToUserMessage>>,
    net: Res<NetworkServer>,
) {
    for message in new_messages.iter() {
        let user = message.source();

        println!("Received message from user: {}", message.message);

        net.broadcast(shared::NewChatMessage {
            name: format!("{}", user),
            message: message.message.clone(),
        });
    }
}