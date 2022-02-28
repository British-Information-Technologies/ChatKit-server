use std::collections::HashMap;
use std::sync::Arc;

use futures::future::{join_all, select};
use tokio::sync::Mutex;
use tokio::select;

use tokio::sync::mpsc::{channel, Receiver, Sender};

use uuid::Uuid;

use async_trait::async_trait;
use tokio::net::ToSocketAddrs;

use foundation::prelude::IManager;
use foundation::ClientDetails;
use foundation::connection::Connection;

use crate::client::Client;
use crate::messages::ClientMessage;
use crate::messages::ClientMgrMessage;
use crate::messages::ServerMessage;

/// # ClientManager
/// This struct manages all connected users
#[derive(Debug)]
pub struct ClientManager<Out: 'static>
	where
		Out: From<ClientMgrMessage> + Send
{
	clients: Mutex<HashMap<Uuid, Arc<Client<ClientMgrMessage>>>>,

	server_channel: Mutex<Sender<Out>>,

	tx: Sender<ClientMgrMessage>,
	rx: Mutex<Receiver<ClientMgrMessage>>,
}

impl<Out> ClientManager<Out>
	where
		Out: From<ClientMgrMessage> + Send
{
	pub fn new(out_channel: Sender<Out>) -> Arc<Self> {
		let (tx, rx) = channel(1024);

		Arc::new(ClientManager {
			clients: Mutex::default(),

			server_channel: Mutex::new(out_channel),

			tx,
			rx: Mutex::new(rx),
		})
	}

	pub async fn get_count(&self) -> usize {
		self.clients.lock().await.len()
	}

	pub async fn add_client(
		&self,
		id: Uuid,
		username: String,
		address: String,
		connection: Arc<Connection>
	) {
		let client = Client::new(
			id,
			username,
			address,
			self.tx.clone(),
			connection
		);
		client.start();
		let mut lock = self.clients.lock().await;
		lock.insert(client.details.uuid, client);
	}

	pub async fn remove_client(&self, id: Uuid) {
		let mut lock = self.clients.lock().await;
		lock.remove(&id);
	}

	pub async fn handle_channel(&self, message: Option<ClientMgrMessage>) {
		use ClientMgrMessage::{Add, Remove, SendClients, BroadcastGlobalMessage, SendError};
		println!("Handling channel");
		match message {
			Some(Add(client)) => {
				let mut lock = self.clients.lock().await;
				lock.insert(client.details.uuid, client);
			},
			Some(Remove(uuid)) => {
				println!("[Client Manager]: removing client: {:?}", &uuid);
				let mut lock = self.clients.lock().await;
				lock.remove(&uuid);
			},
			Some(SendClients { to }) => {
				let lock = self.clients.lock().await;
				if let Some(client) = lock.get(&to) {
					let clients_vec: Vec<ClientDetails> =
						lock.values()
							.cloned()
							.map(|i| i.details.clone())
							.collect();

					// todo: add method to send clients
					// client
					// 	.send_message(ClientMessage::SendClients {
					// 		clients: clients_vec,
					// 	})
					// 	.await
				}
			},
			Some(BroadcastGlobalMessage {sender, content}) => {
				let lock = self.clients.lock().await;
				let futures = lock.iter().map(|(_,_)| async {
					println!("Send message to Client")
				});
				// todo: Implement this instead of prints
				// .map(|i| i.1.send_message(
				// 	ClientMessage::GlobalBroadcastMessage {from: sender, content: content.clone()}
				// ));

				join_all(futures).await;
			},
			Some(SendError { to }) => {
				let lock = self.clients.lock().await;
				if let Some(client) = lock.get(&to) {
					// todo! implement a error message passing function
					// client.send_message(ClientMessage::Error).await
				}
			}
			_ => {
				unimplemented!()
			}
		}
	}

	#[deprecated]
	pub fn start(self: &Arc<ClientManager<Out>>) {
		let client_manager = self.clone();

		tokio::spawn(async move {
			use ClientMgrMessage::{Add, Remove, SendClients, SendError, SendMessage};

			loop {
				let mut receiver = client_manager.rx.lock().await;
				let message = receiver.recv().await.unwrap();

				println!("[Client manager]: received message: {:?}", message);

				match message {
					Add(client) => {
						println!("[Client Manager]: adding new client");
						let mut lock = client_manager.clients.lock().await;
						client.start();
						if lock.insert(client.details.uuid, client).is_none() {
							println!("client added");
						}
					}
					Remove(uuid) => {
						println!("[Client Manager]: removing client: {:?}", &uuid);
						if let Some(client) =
							client_manager.clients.lock().await.remove(&uuid)
						{
							client.send_message(ClientMessage::Disconnect).await;
						}
					}
					// todo: - need to rethink this one
					SendMessage { to, from, content } => {
						client_manager
							.send_to_client(&to, ClientMessage::Message { from, content })
							.await;
					}
					SendClients { to } => {
						let lock = client_manager.clients.lock().await;
						if let Some(client) = lock.get(&to) {
							let clients_vec: Vec<ClientDetails> =
								lock.values().cloned().map(|i| i.details.clone()).collect();

							client
								.send_message(ClientMessage::SendClients {
									clients: clients_vec,
								})
								.await
						}
					}
					ClientMgrMessage::BroadcastGlobalMessage {sender, content} => {
						let lock = client_manager.clients.lock().await;
						let futures = lock.iter()
							.map(|i| i.1.send_message(
								ClientMessage::GlobalBroadcastMessage {from: sender, content: content.clone()}
							));
						
						join_all(futures).await;
					}
					SendError { to } => {
						let lock = client_manager.clients.lock().await;
						if let Some(client) = lock.get(&to) {
							client.send_message(ClientMessage::Error).await
						}
					}
					#[allow(unreachable_patterns)]
					_ => println!("[Client manager]: not implemented"),
				}
			}
		});
	}

	async fn send_to_client(self: &Arc<ClientManager<Out>>, id: &Uuid, msg: ClientMessage) {
		let lock = self.clients.lock().await;
		if let Some(client) = lock.get(&id) {
			client.clone().send_message(msg).await;
		}
	}

	pub async fn send_message(self: Arc<ClientManager<Out>>, message: ClientMgrMessage) {
		let _ = self.tx.send(message).await;
	}
}

#[async_trait]
impl<Out> IManager for ClientManager<Out>
	where
		Out: From<ClientMgrMessage> + Send
{
	async fn run(self: &Arc<Self>) {
		loop {

			let mut receiver = self.rx.lock().await;

			select! {
				val = receiver.recv() => {
					self.handle_channel(val).await;
				}
			}
		}
	}
}


#[cfg(test)]
mod test {
	use std::io::Error;
	use tokio::sync::mpsc::channel;
	use uuid::Uuid;
	use foundation::messages::client::ClientStreamOut;
	use foundation::test::create_connection_pair;
	use crate::client_manager::ClientManager;
	use crate::messages::ClientMgrMessage;

	#[tokio::test]
	async fn add_new_client_to_manager() -> Result<(), Error> {
		let (sender, mut receiver) =
			channel::<ClientMgrMessage>(1024);
		let (server, (client, addr)) = create_connection_pair().await?;

		let client_manager = ClientManager::new(sender);

		let id = Uuid::new_v4();
		let username = "TestUser".to_string();

		client_manager.add_client(
			id,
			username.clone(),
			addr.to_string(),
			server
		).await;

		assert_eq!(client_manager.get_count().await, 1);
		let msg = client.read::<ClientStreamOut>().await?;
		assert_eq!(msg, ClientStreamOut::Connected);

		Ok(())
	}
}
