
use bevy::prelude::*;
use bevy_spicy_networking::{ClientPlugin, NetworkData, NetworkServer, ServerNetworkEvent};
use std::net::SocketAddr;
use super::tracing_logger::LogPlugin;


use super::message_types::server_register_network_messages;

pub struct DebugServer;


impl Plugin for DebugServer {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(bevy_spicy_networking::ServerPlugin);
        app.add_plugin(ClientPlugin);
        server_register_network_messages(app);
        app.add_startup_system(setup_networking.system().label("listen_network"));
        // app.add_plugin(LogPlugin);
        // app.add_system(handle_connection_events.system());
        // app.add_system(handle_messages.system());
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
