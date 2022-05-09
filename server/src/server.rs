use std::io::Error;
use std::sync::Arc;

use uuid::Uuid;

use tokio::sync::{
	mpsc::{channel, Receiver},
	Mutex,
};

// use crate::plugin::{PluginManager, PluginManagerMessage};
use crate::{
	client_manager::{ClientManager, ClientMgrMessage},
	network_manager::{NetworkManager, NetworkManagerMessage},
};

#[derive(Debug, Clone)]
pub enum ServerMessage {
	ClientConnected {
		uuid: Uuid,
		address: String,
		username: String,
		connection: Arc<Connection>,
	},
	BroadcastGlobalMessage {
		from: Uuid,
		content: String,
	},
}

impl From<NetworkManagerMessage> for ServerMessage {
	fn from(msg: NetworkManagerMessage) -> Self {
		use NetworkManagerMessage::ClientConnecting;

		match msg {
			ClientConnecting {
				uuid,
				address,
				username,
				connection,
			} => ServerMessage::ClientConnected {
				uuid,
				address,
				username,
				connection,
			},
			#[allow(unreachable_patterns)]
			_ => unimplemented!(),
		}
	}
}

impl From<ClientMgrMessage> for ServerMessage {
	fn from(msg: ClientMgrMessage) -> Self {
		use ClientMgrMessage::BroadcastGlobalMessage;

		match msg {
			BroadcastGlobalMessage { from, content } => {
				ServerMessage::BroadcastGlobalMessage { from, content }
			}
			_ => unimplemented!(),
		}
	}
}

// impl From<PluginManagerMessage> for ServerMessage {
// 	fn from(_: PluginManagerMessage) -> Self {
// 		todo!()
// 	}
// }

/// # Server
/// authors: @michael-bailey, @Mitch161
/// This Represents a server instance.
/// It is composed of a client manager and a network manager.
///
/// # Attributes
/// - client_manager: The servers client manager.
/// - network_manager: The servers network manager.
/// - receiver: The servers channel for communication by managers.
/// - lua: The servers lua context, used for running lua scripts.
///
pub struct Server {
	pub client_manager: Arc<ClientManager<ServerMessage>>,
	network_manager: Arc<NetworkManager<ServerMessage>>,
	// plugin_manager: Arc<PluginManager<ServerMessage>>,
	receiver: Mutex<Receiver<ServerMessage>>,
}

impl Server {
	/// Create a new server object
	pub async fn new() -> Result<Arc<Server>, Error> {
		let (sender, receiver) = channel(1024);

		let server = Arc::new(Server {
			client_manager: ClientManager::new(sender.clone()),
			network_manager: NetworkManager::new("0.0.0.0:5600", sender.clone()).await?,
			// plugin_manager: PluginManager::new(sender),
			receiver: Mutex::new(receiver),
		});

		Ok(server)
	}

	pub async fn port(self: &Arc<Server>) -> u16 {
		self.network_manager.port().await
	}

	pub async fn start(self: &Arc<Server>) {
		// start client manager and network manager
		self.network_manager.clone().start();
		self.client_manager.clone().start();
		// let _ = self.plugin_manager.clone().load().await;

		// clone block items
		let server = self.clone();

		loop {
			let mut lock = server.receiver.lock().await;
			if let Some(message) = lock.recv().await {
				println!("[server]: received message {:?}", &message);

				match message {
					ServerMessage::ClientConnected {
						uuid,
						address,
						username,
						connection,
					} => {
						server
							.client_manager
							.add_client(uuid, username, address, connection)
							.await
					}
					ServerMessage::BroadcastGlobalMessage {
						from: _,
						content: _,
					} => {
						// server
						// 	.client_manager
						// 	.clone()
						// 	.send_message(
						// 		ClientMgrMessage::BroadcastGlobalMessage {sender, content}
						// 	).await
					}
					#[allow(unreachable_patterns)]
					_ => {
						unimplemented!()
					}
				}
			}
		}
	}
}
