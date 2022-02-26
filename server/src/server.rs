use std::io::Error;
use std::sync::Arc;

// use crossbeam_channel::{unbounded, Receiver};
use futures::lock::Mutex;
use tokio::sync::mpsc::{channel, Receiver};
use foundation::prelude::IManager;

use crate::client_manager::ClientManager;
use crate::messages::{ClientMessage, ClientMgrMessage};
use crate::messages::ServerMessage;
use crate::network_manager::{NetworkManager, NetworkManagerMessage};

impl From<NetworkManagerMessage> for ServerMessage {
	fn from(_: NetworkManagerMessage) -> Self {
		ServerMessage::Some
	}
}


/// # Server
/// authors: @michael-bailey, @Mitch161
/// This Represents a server instance.
/// it is componsed of a client manager and a network manager
///
pub struct Server {
	client_manager: Arc<ClientManager>,
	network_manager: Arc<NetworkManager<ServerMessage>>,
	receiver: Mutex<Receiver<ServerMessage>>,
}

impl Server {
	/// Create a new server object
	pub async fn new() -> Result<Arc<Server>, Error> {
		let (
			sender,
			receiver
		) = channel(1024);

		Ok(Arc::new(Server {
			client_manager: ClientManager::new(sender.clone()),
			network_manager: NetworkManager::new("0.0.0.0:5600", sender).await?,
			receiver: Mutex::new(receiver),
		}))
	}

	pub async fn port(self: &Arc<Server>) -> u16 {
		self.network_manager.port().await
	}
	
	pub async fn start(self: &Arc<Server>) {
		// start client manager and network manager
		self.network_manager.clone().start();
		self.client_manager.clone().start();

		// clone block items
		let server = self.clone();

		use ClientMgrMessage::{Add, Remove, SendMessage};

		loop {
			let mut lock = server.receiver.lock().await;
			if let Some(message) = lock.recv().await {
				println!("[server]: received message {:?}", &message);

				match message {
					ServerMessage::ClientConnected { client } => {
						server
							.client_manager.add_client()

							// .send_message(Add(client))
							.await
					},
					ServerMessage::ClientDisconnected { id } => {
						println!("disconnecting client {:?}", id);
						server.client_manager.clone().send_message(Remove(id)).await;
					}
					ServerMessage::ClientSendMessage { from, to, content } => {
						server
							.client_manager
							.clone()
							.send_message(SendMessage { from, to, content })
							.await
					}
					ServerMessage::ClientUpdate { to } => {
						server
							.client_manager
							.clone()
							.send_message(ClientMgrMessage::SendClients { to })
							.await
					}
					ServerMessage::ClientError { to } => {
						server
							.client_manager
							.clone()
							.send_message(ClientMgrMessage::SendError { to })
							.await
					}
					ServerMessage::BroadcastGlobalMessage {sender,content} => {
						server
							.client_manager
							.clone()
							.send_message(
								ClientMgrMessage::BroadcastGlobalMessage {sender, content}
							).await
					}
					_ => {unimplemented!()}
				}
			}
		}
	}
}
