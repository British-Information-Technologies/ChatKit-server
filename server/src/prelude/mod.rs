//! # prelude
//! A module that coalesces different types into one module of defined structure

mod observer;

pub mod actors {
	//! exports all actors used in the program.
	pub use crate::server::Server;
	pub(crate) use crate::network::{Connection, ConnectionInitiator, NetworkManager};
	pub(crate) use crate::client_management::{Client,ClientManager};
}
pub mod messages {
	//! exports all messages used in the program.
	pub(crate) use super::observer::ObservableMessage;
	pub(crate) use crate::network::{NetworkMessage,NetworkOutput,ConnectionMessage,ConnectionOuput};
	pub(crate) use crate::client_management::{ClientManagerOutput,ClientManagerMessage};

}