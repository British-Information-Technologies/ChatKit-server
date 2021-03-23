use foundation::prelude::IPreemptive;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::net::TcpListener;
use std::sync::Arc;
use std::thread;

use crossbeam_channel::Sender;

use crate::client::Client;
use crate::messages::ServerMessage;
use foundation::messages::network::{NetworkSockIn, NetworkSockOut};

pub struct NetworkManager {
	listener: TcpListener,
	server_channel: Sender<ServerMessage>,
}

impl NetworkManager {
	pub fn new(
		port: String,
		server_channel: Sender<ServerMessage>,
	) -> Arc<NetworkManager> {
		let mut address = "0.0.0.0:".to_string();
		address.push_str(&port);

		let listener =
			TcpListener::bind(address).expect("Could not bind to address");

		Arc::new(NetworkManager {
			listener,
			server_channel,
		})
	}
}

impl IPreemptive for NetworkManager {
	fn start(arc: &Arc<Self>) {
		let arc = arc.clone();
		std::thread::spawn(move || {
			// fetch new connections and add them to the client queue
			for connection in arc.listener.incoming() {
				println!("[NetworkManager]: New Connection!");
				match connection {
					Ok(stream) => {
						let server_channel = arc.server_channel.clone();

						// create readers
						let mut reader =
							BufReader::new(stream.try_clone().unwrap());
						let mut writer =
							BufWriter::new(stream.try_clone().unwrap());

						let _handle = thread::Builder::new()
							.name("NetworkJoinThread".to_string())
							.spawn(move || {
								let mut buffer = String::new();

								// send request message to connection
								writer
									.write_all(
										serde_json::to_string(
											&NetworkSockOut::Request,
										)
										.unwrap()
										.as_bytes(),
									)
									.unwrap_or_default();
								writer.write_all(b"\n").unwrap_or_default();
								writer.flush().unwrap_or_default();

								// try get response
								let res = reader.read_line(&mut buffer);
								if res.is_err() {
									return;
								}

								//match the response
								if let Ok(request) = serde_json::from_str::<
									NetworkSockIn,
								>(&buffer)
								{
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
											writer.write_all(b"\n").unwrap();
											writer.flush().unwrap();
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
												stream.try_clone().unwrap(),
												server_channel.clone(),
											);
											server_channel.send(
                    ServerMessage::ClientConnected(new_client)
                  ).unwrap_or_default();
										}
									}
								}
							});
					}
					Err(e) => {
						println!(
							"[Network manager]: error getting stream: {:?}",
							e
						);
						continue;
					}
				}
			}
		});
	}
}
