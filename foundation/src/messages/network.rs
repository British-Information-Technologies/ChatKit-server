use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NetworkSockIn {
	Info,
	Connect {
		uuid: String,
		username: String,
		address: String,
	},
}

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
