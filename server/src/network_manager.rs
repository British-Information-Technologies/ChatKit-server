use std::io::Write;
use std::sync::Arc;

use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::mpsc::Sender;
use tokio::task;

use crate::client::Client;
use crate::messages::ServerMessage;
use foundation::messages::network::{NetworkSockIn, NetworkSockOut};

pub struct NetworkManager {
	address: String,
	server_channel: Sender<ServerMessage>,
}

impl NetworkManager {
	pub fn new(
		_port: String,
		server_channel: Sender<ServerMessage>,
	) -> Arc<NetworkManager> {
		Arc::new(NetworkManager {
			address: "0.0.0.0:5600".to_string(),
			server_channel,
		})
	}

	pub fn start(self: &Arc<NetworkManager>) {
		let network_manager = self.clone();

		tokio::spawn(async move {
			let listener = TcpListener::bind(network_manager.address.clone())
				.await
				.unwrap();

			loop {
				let (connection, _) = listener.accept().await.unwrap();
				let (rd, mut wd) = io::split(connection);

				let mut reader = BufReader::new(rd);
				let server_channel = network_manager.server_channel.clone();

				task::spawn(async move {
					let mut out_buffer: Vec<u8> = Vec::new();
					let mut in_buffer: String = String::new();

					// write request
					let a = serde_json::to_string(&NetworkSockOut::Request).unwrap();
					println!("{:?}", &a);
					let _ = writeln!(out_buffer, "{}", a);

					let _ = wd.write_all(&out_buffer).await;
					let _ = wd.flush().await;

					// get response
					let _ = reader.read_line(&mut in_buffer).await.unwrap();

					//match the response
					if let Ok(request) = serde_json::from_str::<NetworkSockIn>(&in_buffer)
					{
						match request {
							NetworkSockIn::Info => {
								// send back server info to the connection
								let _ = wd
									.write_all(
										serde_json::to_string(&NetworkSockOut::GotInfo {
											server_name: "oof",
											server_owner: "michael",
										})
										.unwrap()
										.as_bytes(),
									)
									.await;
								let _ = wd.write_all(b"\n").await;
								let _ = wd.flush().await;
							}
							NetworkSockIn::Connect {
								uuid,
								username,
								address,
							} => {
								// create client and send to server
								let new_client = Client::new(
									uuid,
									username,
									address,
									reader,
									wd,
									server_channel.clone(),
								);
								let _ = server_channel
									.send(ServerMessage::ClientConnected {
										client: new_client,
									})
									.await;
							}
						}
					}
				});
			}
		});
	}
}
