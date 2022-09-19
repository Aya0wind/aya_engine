
use bevy::prelude::*;
use bevy_spicy_networking::{ConnectionId, NetworkData, NetworkServer, ServerNetworkEvent};
use std::net::SocketAddr;
use super::tracing_logger::LogPlugin;

use crate::log::message_types;

use super::message_types::server_register_network_messages;

pub struct DebugServer;


impl Plugin for DebugServer {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(bevy_spicy_networking::ServerPlugin);
        server_register_network_messages(app);
        app.add_startup_system(setup_networking.system().label("listen_network"));
        app.add_plugin(LogPlugin);
        app.add_system(handle_connection_events.system());
        app.add_system(handle_messages.system());
    }
}

// On the server side, you need to setup networking. You do not need to do so at startup, and can start listening
// at any time.
fn setup_networking(mut net: ResMut<NetworkServer>) {
    let ip_address = "127.0.0.1".parse().expect("Could not parse ip address");

    let socket_address = SocketAddr::new(ip_address, 9999);

    match net.listen(socket_address) {
        Ok(_) => (),
        Err(err) => {
            error!("Could not start listening: {:?}", err);
            panic!();
        }
    }
}

struct Console(ConnectionId);

fn handle_connection_events(
    mut commands: Commands,
    net: Res<NetworkServer>,
    mut network_events: EventReader<ServerNetworkEvent>,
) {
    for event in network_events.iter() {
        if let ServerNetworkEvent::Connected(conn_id) = event {
            commands.spawn_bundle((Console(*conn_id), ));

            // Broadcasting sends the message to all connected players! (Including the just connected one in this case)
            net.broadcast(message_types::SendToUserMessage {
                message: format!("New console connected; {:?}", conn_id),
            });
        }
    }
}

// Receiving a new message is as simple as listening for events of `NetworkData<T>`
fn handle_messages(
    mut new_messages: EventReader<NetworkData<message_types::SendToUserMessage>>,
    net: Res<NetworkServer>,
) {
    for message in new_messages.iter() {
        let user = message.source();

        println!("Received message from user: {}", message.message);

        net.broadcast(message_types::SendToUserMessage {
            message: message.message.clone(),
        });
    }
}
