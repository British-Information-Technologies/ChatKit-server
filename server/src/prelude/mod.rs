//! # prelude
//! A module that coalesces different types into one module of defined structure

mod observer;

pub mod actors {
	//! exports all actors used in the program.
	pub use crate::server::Server;
	pub(crate) use crate::network::{Connection, ConnectionInitiator, NetworkManager};
	pub(crate) use crate::client_management::ClientManager;
	pub(crate) use crate::client_management::client::Client;
}
pub mod messages {
	//! exports all messages used in the program.
	pub(crate) use super::observer::ObservableMessage;
	pub(crate) use crate::network::{ConnectionMessage, ConnectionOuput, NetworkMessage, NetworkOutput};
	pub(crate) use crate::client_management::{ClientManagerMessage, ClientManagerOutput};

}