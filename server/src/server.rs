use std::sync::Arc;

// use crossbeam_channel::{unbounded, Receiver};
use uuid::Uuid;
use tokio::sync::mpsc::{channel, Receiver};
use futures::lock::Mutex;

use crate::client_manager::ClientManager;
use crate::messages::ClientMgrMessage;
use crate::messages::ServerMessage;
use crate::network_manager::NetworkManager;

/// # ServerMessages
/// This is used internally to send messages to the server to be dispatched
#[derive(Debug)]
pub enum ServerMessages<TClient> {
	ClientConnected(Arc<TClient>),
	ClientDisconnected(Uuid),
}

/// # Server
/// authors: @michael-bailey, @Mitch161
/// This Represents a server instance.
/// it is componsed of a client manager and a network manager
/// 
pub struct Server {
	client_manager: Arc<ClientManager>,
	network_manager: Arc<NetworkManager>,
	receiver: Mutex<Receiver<ServerMessage>>,
}

impl Server {
	/// Create a new server object
	pub fn new() -> Result<Arc<Server>, Box<dyn std::error::Error>> {
		let (sender, receiver) = channel(1024);

		Ok(
			Arc::new(
				Server {
					client_manager: ClientManager::new(sender.clone()),
					network_manager: NetworkManager::new("5600".to_string(), sender),
					receiver: Mutex::new(receiver),
				}
			)
		)
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
						server.client_manager.clone()
						.send_message(Add(client)).await
					}
					ServerMessage::ClientDisconnected { id } => {
						println!("disconnecting client {:?}", id);
						server.client_manager.clone().send_message(Remove(id)).await;
					}
					ServerMessage::ClientSendMessage { from, to, content } => server
						.client_manager.clone()
						.send_message(SendMessage { from, to, content }).await,
					ServerMessage::ClientUpdate { to } => server
						.client_manager.clone()
						.send_message(ClientMgrMessage::SendClients { to }).await,
					ServerMessage::ClientError { to } => server
						.client_manager.clone()
						.send_message(ClientMgrMessage::SendError {to}).await,
				}
			}
		}
	}
}

