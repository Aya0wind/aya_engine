use bevy::prelude::*;
use bevy_spicy_networking::{ClientMessage, NetworkMessage, ServerMessage};
use serde::{Deserialize, Serialize};

/////////////////////////////////////////////////////////////////////
// In this example the client sends `UserChatMessage`s to the server,
// the server then broadcasts to all connected clients.
//
// We use two different types here, because only the server should
// decide the identity of a given connection and thus also sends a
// name.
//
// You can have a single message be sent both ways, it simply needs
// to implement both `ClientMessage` and `ServerMessage`.
/////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SendToUserMessage {
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SendToServerMessage {
    pub message: String,
}


#[typetag::serde]
impl NetworkMessage for SendToUserMessage {}

impl ClientMessage for SendToUserMessage {
    const NAME: &'static str = "example:NewServerMessage";
}

#[typetag::serde]
impl NetworkMessage for SendToServerMessage {}

impl ServerMessage for SendToServerMessage {
    const NAME: &'static str = "example:NewClientMessage";
}

#[allow(unused)]
pub fn client_register_network_messages(app: &mut AppBuilder) {
    use bevy_spicy_networking::AppNetworkClientMessage;

    // The client registers messages that arrives from the server, so that
    // it is prepared to handle them. Otherwise, an error occurs.
    app.listen_for_client_message::<SendToUserMessage>();
}

#[allow(unused)]
pub fn server_register_network_messages(app: &mut AppBuilder) {
    use bevy_spicy_networking::AppNetworkServerMessage;

    // The server registers messages that arrives from a client, so that
    // it is prepared to handle them. Otherwise, an error occurs.
    app.listen_for_server_message::<SendToServerMessage>();
}