use foundation::ClientDetails;
use foundation::messages::network::NetworkSockOut;

#[derive(Debug)]
pub enum NetworkManagerMessage {
	Users(Vec<ClientDetails>),
	Info {
		server_name: String,
		server_owner: String,
	},
	Error(&'static str)
}

impl From<NetworkSockOut> for NetworkManagerMessage {
	fn from(other: NetworkSockOut) -> Self {
		use NetworkSockOut::{GotInfo as OldInfo};
		use NetworkManagerMessage::{Info as NewInfo, Error};
		match other {
			OldInfo {server_name,server_owner} => NewInfo {server_name,server_owner},
			_ => Error("Error occurred with conversion")
		}
	}
}

impl PartialEq for NetworkManagerMessage {
	fn eq(&self, other: &Self) -> bool {
		use NetworkManagerMessage::Info;
		match self {
			Info {server_owner, server_name} => {
				if let Info {server_owner: other_owner,server_name: other_name} = other {
					return server_owner == other_owner && server_name == other_name;
				}
				false
			}
			_ => {false}
		}
	}
}