use std::{collections::HashMap, net::SocketAddr};

use foundation::prelude::{ClientDetails, GlobalMessage};
use tokio::sync::{
	mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
	Mutex,
};
use uuid::Uuid;

use crate::{
	connection::{client_info::ClientInfo, client_thread::ClientThread},
	network::NetworkConnection,
	server_va::ServerMessages,
};

pub struct ConnectionManager {
	receiver: Mutex<UnboundedReceiver<ConnectionManagerMessage>>,
	sender: UnboundedSender<ConnectionManagerMessage>,
	server_sender: UnboundedSender<ServerMessages>,
	client_map: HashMap<Uuid, ClientInfo>,
	client_tasks_map: HashMap<Uuid, ClientThread>,
}

impl ConnectionManager {
	pub fn new(server_sender: UnboundedSender<ServerMessages>) -> Self {
		let (tx, rx) = unbounded_channel();
		Self {
			client_map: HashMap::new(),
			client_tasks_map: HashMap::new(),
			server_sender,
			receiver: Mutex::new(rx),
			sender: tx,
		}
	}

	pub async fn run(&mut self) {
		loop {
			let mut lock = self.receiver.lock().await;
			let msg = lock.recv().await;
			drop(lock);

			match msg {
				Some(ConnectionManagerMessage::AddClient {
					conn,
					uuid,
					username,
					addr,
				}) => self.add_client(conn, uuid, username, addr).await,

				Some(ConnectionManagerMessage::Disconnected { uuid }) => {
					self.remove_client(uuid).await
				}
				Some(ConnectionManagerMessage::BroadcastGlobalMessage {
					from,
					content,
				}) => {
					self.broadcast_global_message(from, content).await;
				}
				Some(ConnectionManagerMessage::SendClientsTo { uuid }) => {
					self.send_clients_to(uuid).await;
				}
				Some(ConnectionManagerMessage::SendGlobalMessages { uuid }) => {
					self.send_global_messages(uuid).await;
				}

				Some(ConnectionManagerMessage::SendGlobalMessagesTo {
					uuid,
					messages,
				}) => {
					self.send_global_messages_to(uuid, messages).await;
				}

				Some(ConnectionManagerMessage::SendPrivateMessage {
					uuid,
					from,
					to,
					content,
				}) => {
					self.send_private_message(to, from, uuid, content).await;
				}
				Some(ConnectionManagerMessage::Disconnect { uuid }) => {
					self.disconnect(uuid).await
				}
				None => todo!(),
			}
		}
	}

	async fn add_client(
		&mut self,
		conn: Box<dyn NetworkConnection>,
		uuid: Uuid,
		username: String,
		addr: SocketAddr,
	) {
		println!("[ConnectionManager] adding new client");
		let store = ClientInfo::new(uuid, username.clone(), addr);
		self.client_map.insert(uuid, store);
		println!("[ConnectionManager] added client info to map");

		let thread = ClientThread::new_run(uuid, conn, self.sender.clone()).await;
		self.client_tasks_map.insert(uuid, thread);
		println!("[ConnectionManager] created running thread for new clinet");

		for c in self.client_tasks_map.iter_mut() {
			c.1
				.send_client_joined(ClientDetails {
					uuid: uuid.to_string(),
					name: username.clone(),
					address: addr.to_string(),
				})
				.await;
		}
	}

	async fn remove_client(&mut self, uuid: Uuid) {
		println!("[ConnectionManager] removing {}", uuid);
		self.client_map.remove(&uuid);
		self.client_tasks_map.remove(&uuid);

		for c in self.client_tasks_map.iter_mut() {
			c.1.send_client_left(uuid).await;
		}
	}

	async fn send_clients_to(&mut self, uuid: Uuid) {
		let clients = self
			.client_map
			.values()
			.cloned()
			.map(|c| foundation::prelude::ClientDetails {
				uuid: c.get_uuid().to_string(),
				name: c.get_username(),
				address: c.get_addr().to_string(),
			})
			.collect();

		let t = self.client_tasks_map.get_mut(&uuid);
		let Some(t) = t else {
			return;
		};

		println!("[ConnectionManager] sending client list to {:?}", clients);

		t.send_clients(clients).await;
	}

	async fn broadcast_global_message(&mut self, from: Uuid, content: String) {
		let message = GlobalMessage {
			uuid: Uuid::new_v4().to_string(),
			from: from.to_string(),
			content,
		};
		_ = self
			.server_sender
			.send(ServerMessages::AddGlobalMessage(message.clone()));
		for c in self.client_tasks_map.iter_mut() {
			c.1.send_global_message(message.clone()).await;
		}
	}

	async fn send_global_messages(&mut self, uuid: Uuid) {
		_ = self
			.server_sender
			.send(ServerMessages::SendGlobalMessages(uuid));
	}

	async fn send_global_messages_to(
		&mut self,
		uuid: Uuid,
		messages: Vec<GlobalMessage>,
	) {
		let t = self.client_tasks_map.get_mut(&uuid);
		let Some(t) = t else {
			return;
		};

		t.send_global_messages(messages).await;
	}

	async fn send_private_message(
		&mut self,
		to: Uuid,
		from: Uuid,
		uuid: Uuid,
		content: String,
	) {
		let t = self.client_tasks_map.get_mut(&to);
		let Some(t) = t else {
			return;
		};

		t.send_private_message(from, uuid, content).await
	}

	async fn disconnect(&mut self, uuid: Uuid) {
		let t = self.client_tasks_map.get_mut(&uuid);
		let Some(t) = t else {
			return;
		};

		t.send_disconnected().await;
	}

	pub fn get_sender(&self) -> UnboundedSender<ConnectionManagerMessage> {
		self.sender.clone()
	}
}

pub enum ConnectionManagerMessage {
	// server messages
	AddClient {
		conn: Box<dyn NetworkConnection + 'static>,
		uuid: Uuid,
		username: String,
		addr: SocketAddr,
	},

	// client thread messages
	SendClientsTo {
		uuid: Uuid,
	},

	SendGlobalMessages {
		uuid: Uuid,
	},

	SendGlobalMessagesTo {
		uuid: Uuid,
		messages: Vec<GlobalMessage>,
	},

	BroadcastGlobalMessage {
		from: Uuid,
		content: String,
	},

	SendPrivateMessage {
		uuid: Uuid,
		from: Uuid,
		to: Uuid,
		content: String,
	},

	Disconnect {
		uuid: Uuid,
	},

	Disconnected {
		uuid: Uuid,
	},
}
