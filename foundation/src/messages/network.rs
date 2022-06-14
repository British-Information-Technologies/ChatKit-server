use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Message the server will receive from a socket
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NetworkSockIn {
	Info,
	Connect {
		uuid: Uuid,
		username: String,
		address: String,
	},
}

/// Message the server will send through a socket
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum NetworkSockOut {
	Request,

	GotInfo {
		server_name: String,
		server_owner: String,
	},
	Connecting,
	
	Error
}

impl PartialEq for NetworkSockOut {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(NetworkSockOut::Request, NetworkSockOut::Request) => true,
			(NetworkSockOut::GotInfo {server_name,server_owner},
				NetworkSockOut::GotInfo {server_owner: owner_other,server_name: name_other})
					=> server_name == name_other && server_owner == owner_other,
			(NetworkSockOut::Connecting, NetworkSockOut::Connecting) => true,
			_ => false
		}
	}
}
