use crate::managers::NetworkManagerMessage;

pub enum WorkerMessage {
	Info {
		server_name: String,
		server_owner: String,
	},
	Error(&'static str),
}

impl From<NetworkManagerMessage> for WorkerMessage {
	fn from(other: NetworkManagerMessage) -> Self {
		use WorkerMessage::{Info as NewInfo, Error as NewError};
		use NetworkManagerMessage::{Info as OldInfo, Error};
		match other {
			OldInfo {server_name, server_owner}
				=> NewInfo {server_owner,server_name},
			_ => todo!()
		}
	}
}
