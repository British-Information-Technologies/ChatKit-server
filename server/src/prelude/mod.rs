//! # prelude
//! A module that coalesces different types into one module of defined structure

mod observer;

#[allow(unused_imports)]
pub mod actors {
	//! exports all actors used in the program.
	pub use crate::server::Server;
	pub(crate) use crate::{
		client_management::{client::Client, ClientManager},
		network::{Connection, ConnectionInitiator, NetworkManager},
	};
}

#[allow(unused_imports)]
pub mod messages {
	//! exports all messages used in the program.
	pub(crate) use super::observer::ObservableMessage;
	pub(crate) use crate::{
		client_management::{ClientManagerMessage, ClientManagerOutput},
		network::{
			ConnectionMessage,
			ConnectionObservableOutput,
			NetworkMessage,
			NetworkOutput,
		},
	};
}
