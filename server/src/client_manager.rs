use std::collections::HashMap;
use std::sync::Arc;

use futures::future::{join_all, select};
use tokio::sync::Mutex;
use tokio::select;

use tokio::sync::mpsc::{channel, Receiver, Sender};

use uuid::Uuid;

use async_trait::async_trait;

use foundation::prelude::IManager;
use foundation::ClientDetails;

use crate::client::Client;
use crate::messages::ClientMessage;
use crate::messages::ClientMgrMessage;
use crate::messages::ServerMessage;

/// # ClientManager
/// This struct manages all connected users
#[derive(Debug)]
pub struct ClientManager<Out>
	where
		Out: From<ClientMgrMessage>
{
	clients: Mutex<HashMap<Uuid, Arc<Client<ClientMgrMessage>>>>,

	server_channel: Mutex<Sender<Out>>,

	tx: Sender<ClientMgrMessage>,
	rx: Mutex<Receiver<ClientMgrMessage>>,
}

impl<Out> ClientManager<Out>
	where
		Out: From<ClientMgrMessage>
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

	pub async fn handle_channel(&self, message: Option<ClientMgrMessage>) {
		println!("Handling channel")
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
		Out: From<ClientMgrMessage>
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
	use crate::client_manager::ClientManager;

	async fn create_new_client_manager() -> Result<(), Error> {
		let client_manager = ClientManager::new()
	}
}

