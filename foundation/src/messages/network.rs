use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum NetworkSockIn {
	Info,
	Connect {
		uuid: String,
		username: String,
		address: String,
	},
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum NetworkSockOut<'a> {
	Request,

	GotInfo {
		server_name: &'a str,
		server_owner: &'a str,
	},
	Connecting,
}
