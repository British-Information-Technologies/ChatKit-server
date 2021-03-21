use crate::client::Client;
use crate::messages::ServerMessage;
use std::io::BufWriter;
use std::io::BufReader;
use std::sync::Arc;
use crate::server::ServerMessages;
use std::net::TcpListener;
use std::io::Write;
use std::io::BufRead;

use crossbeam_channel::{Sender};

use foundation::prelude::ICooperative;
use foundation::messages::network::{NetworkSockIn, NetworkSockOut};

pub struct NetworkManager {
  listener: TcpListener,
  server_channel: Sender<ServerMessage>,
}

impl NetworkManager {
  pub fn new(
		port: String, 
		server_channel: Sender<ServerMessage>
	) -> Arc<NetworkManager> {
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

				// send request message to connection
				writer.write_all(
					serde_json::to_string(&NetworkSockOut::Request).unwrap().as_bytes()
				).unwrap_or_default();
				writer.write_all(b"\n").unwrap_or_default();
				writer.flush().unwrap_or_default();

				// read the new request into a buffer
				let res = reader.read_line(&mut buffer);

        // if reading caused an error skip the connection
				if res.is_err() {continue;}

				// turn into enum and perform pattern matching
				if let Ok(request) = serde_json::from_str::<NetworkSockIn>(&buffer) {
					match request {
						NetworkSockIn::Info => {
              // send back server info to the connection
							writer.write_all(
								serde_json::to_string(
									&NetworkSockOut::GotInfo {
										server_name: "oof", 
										server_owner: "michael"
									}
								).unwrap().as_bytes()
							).unwrap();
							writer.flush().unwrap();
						}
						NetworkSockIn::Connect { uuid, username, address } => {
              // create client and send to server
							let new_client = Client::new(
                uuid,
								username,
								address,
								stream.try_clone().unwrap(),
								self.server_channel.clone()
							);
							self.server_channel.send(
								ServerMessage::ClientConnected(new_client)
							).unwrap_or_default();
						}
					}
				}
			}
		}
  }
}