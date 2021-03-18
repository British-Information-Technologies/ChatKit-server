
use std::net::TcpListener;
use std::sync::Arc;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::io::BufRead;

use serde::{Deserialize, Serialize};
use crossbeam_channel::Sender;

use crate::lib::server::ServerMessages;
use crate::lib::Foundation::ICooperative;


/// # NetworkSockIn
/// these messages can be sent by a client on connecting
#[derive(Serialize, Deserialize)]
enum NetworkSockIn {
  Info,
  Connect {uuid: String, username: String, address: String},
}

/// # NetworkSockOut
/// these messages are sent by the network manager on connecting and requesting
#[derive(Serialize, Deserialize)]
enum NetworkSockOut<'a> {
	Request,
	GotInfo {server_name: &'a str, server_owner: &'a str}
}

// these are control signals from the server.
pub enum NetworkMessages {

}

pub struct NetworkManager {
  listener: TcpListener,
  server_channel: Sender<ServerMessages>,
}

impl NetworkManager {
  pub fn new(port: String, server_channel: Sender<ServerMessages>) -> Arc<NetworkManager> {
    let mut address = "0.0.0.0:".to_string();
    address.push_str(&port);

		let listener = TcpListener::bind(address)
      .expect("Could not bind to address");

    Arc::new(NetworkManager {
      listener,
      server_channel,
    })
  }
}

impl ICooperative for NetworkManager {
  fn tick(&self) {
    // get all new connections
		// handle each request
		for connection in self.listener.incoming() {
			if let Ok(stream) = connection {

				// create buffered writers
				let mut reader = BufReader::new(stream.try_clone().unwrap());
				let mut writer = BufWriter::new(stream.try_clone().unwrap());

				let mut buffer = String::new();

				// request is always sent on new connection
				writer.write_all(serde_json::to_string(&NetworkSockOut::Request).unwrap().as_bytes()).unwrap_or_default();
				writer.write_all(b"\n").unwrap_or_default();
				writer.flush().unwrap_or_default();

				// read the new request into a buffer
				let res = reader.read_line(&mut buffer);
				if res.is_err() {continue;}

				// turn into enum for pattern matching
				if let Ok(request) = serde_json::from_str::<NetworkSockIn>(&buffer) {
					// perform action based on the enum
					match request {
						NetworkSockIn::Info => {
							writer.write_all(
								serde_json::to_string(
									&NetworkSockOut::GotInfo {server_name: "oof", server_owner: "michael"}
								).unwrap().as_bytes()
							).unwrap();
							writer.flush().unwrap();
						}
						NetworkSockIn::Connect { uuid, username, address } => {
							println!("Connection requested")
						}
					}
				}
			}
		}
  }
}