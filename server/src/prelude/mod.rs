//! # prelude
//! A module that coalesces different types into one module of defined structure

mod observer;

#[allow(unused_imports)]
pub mod actors {
	//! exports all actors used in the program.
	pub(crate) use crate::client_management::client::Client;
	pub(crate) use crate::client_management::ClientManager;
	pub(crate) use crate::network::{
		Connection, ConnectionInitiator, NetworkManager,
	};
	pub use crate::server::Server;
}

#[allow(unused_imports)]
pub mod messages {
	//! exports all messages used in the program.
	pub(crate) use super::observer::ObservableMessage;
	pub(crate) use crate::client_management::{
		ClientManagerMessage, ClientManagerOutput,
	};
	pub(crate) use crate::network::{
		ConnectionMessage, ConnectionOuput, NetworkMessage, NetworkOutput,
	};
}
