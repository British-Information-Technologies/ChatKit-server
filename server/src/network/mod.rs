//! # Network
//!
//! This module contains network code for the server.
//!
//! This includes:
//! - The network manager: For that handles all server network connections.
//! - The network listener: For listening for connections on a port.
//! - The conneciton: An abstraction over sockets sockets, for actix.
//! - The connection initiator: For initiating new connections to the server
//!
//! ## Diagrams
//!
//! ```mermaid
//! sequenceDiagram
//!		Server->>NetworkManager: creates
//!		NetworkManager->>NetworkListener: create
//!		NetworkManager->>+NetworkListener: start listening
//!
//!		loop async tcp listen
//!				NetworkListener->>NetworkListener: check for new connections
//!		end
//!
//!		NetworkListener->>Connection: create from socket
//!		NetworkListener->>NetworkManager: new connection
//!		NetworkManager->>Server: new connection
//!
//!		Server->>ConnectionInitiator: create with connection
//! ```

mod connection;
mod connection_initiator;
mod listener;
mod network_manager;

pub(crate) use connection::{Connection, ConnectionMessage, ConnectionOuput};
pub(crate) use connection_initiator::{ConnectionInitiator, InitiatorOutput};
// use listener::{ListenerMessage, ListenerOutput, NetworkListener};
pub(crate) use network_manager::{
	NetworkDataMessage, NetworkDataOutput, NetworkManager, NetworkMessage,
	NetworkOutput,
};
