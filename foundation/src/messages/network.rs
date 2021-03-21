use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub enum NetworkSockIn {
  Info,
  Connect {uuid: String, username: String, address: String},
}

#[derive(Serialize, Deserialize)]
pub enum NetworkSockOut<'a> {
	Request,
  
	GotInfo {server_name: &'a str, server_owner: &'a str},
  Connecting,
}