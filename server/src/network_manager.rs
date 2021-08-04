use std::sync::Arc;

use tokio::net::TcpListener;
use tokio::sync::mpsc::Sender;

use crate::client::Client;
use crate::network::SocketSender;
use crate::messages::ServerMessage;
use crate::prelude::StreamMessageSender;
use foundation::messages::network::{NetworkSockIn, NetworkSockOut};

pub struct NetworkManager {
	address: String,
	server_channel: Sender<ServerMessage>,
}

impl NetworkManager {
	pub fn new(_port: String, server_channel: Sender<ServerMessage>) -> Arc<NetworkManager> {
		Arc::new(NetworkManager {
			address: "0.0.0.0:5600".to_string(),
			server_channel,
		})
	}

	pub fn start(self: &Arc<NetworkManager>) {

		let network_manager = self.clone();

		tokio::spawn(async move {
			let listener = TcpListener::bind(network_manager.address.clone()).await.unwrap();

			loop {
				let (connection, _) = listener.accept().await.unwrap();
				let stream_sender = SocketSender::new(connection);
				let server_channel = network_manager.server_channel.clone();

				tokio::spawn(async move {

					stream_sender.send::<NetworkSockOut>(NetworkSockOut::Request)
						.await.expect("failed to send message");

					if let Ok(request) = 
						stream_sender.recv::<NetworkSockIn>().await 
					{

						match request {
							NetworkSockIn::Info => {
								stream_sender.send(
									NetworkSockOut::GotInfo {
										server_name: "oof",
										server_owner: "michael",
									}
								).await.expect("failed to send got info");
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
									stream_sender,
									server_channel.clone(),
								);
								let _ = server_channel
									.send(ServerMessage::ClientConnected {
										client: new_client,
									}).await;
							}
						}
					}
				});
			}
		});	
	}
}
