use std::{collections::HashMap, net::SocketAddr};

use tokio::sync::{
	mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
	Mutex,
};
use uuid::Uuid;

use crate::{
	connection::{client_info::ClientInfo, client_thread::ClientThread},
	network::network_connection::NetworkConnection,
};

pub struct ConnectionManager {
	receiver: Mutex<UnboundedReceiver<ConnectionManagerMessage>>,
	sender: UnboundedSender<ConnectionManagerMessage>,
	client_map: HashMap<Uuid, ClientInfo>,
	client_tasks_map: HashMap<Uuid, ClientThread>,
}

impl ConnectionManager {
	pub fn new() -> Self {
		let (tx, rx) = unbounded_channel();
		Self {
			client_map: HashMap::new(),
			client_tasks_map: HashMap::new(),
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
				Some(_) => {}
				None => todo!(),
			}
		}
	}

	async fn add_client(
		&mut self,
		conn: NetworkConnection,
		uuid: Uuid,
		username: String,
		addr: SocketAddr,
	) {
		let store = ClientInfo::new(uuid, username, addr);
		self.client_map.insert(uuid, store);

		let thread = ClientThread::new_run(uuid, conn, self.sender.clone()).await;

		self.client_tasks_map.insert(uuid, thread);
	}

	pub fn get_sender(&self) -> UnboundedSender<ConnectionManagerMessage> {
		self.sender.clone()
	}
}

impl Default for ConnectionManager {
	fn default() -> Self {
		Self::new()
	}
}

pub enum ConnectionManagerMessage {
	// server messages
	AddClient {
		conn: NetworkConnection,
		uuid: Uuid,
		username: String,
		addr: SocketAddr,
	},

	// client thread messages
	SendClientsTo {
		uuid: Uuid,
	},

	SendGlobalMessagesTo {
		uuid: Uuid,
	},

	BroadcastGlobalMessage {
		from: Uuid,
		content: String,
	},

	SendPrivateMessage {
		uuid: String,
		from: Uuid,
		to: Uuid,
		content: String,
	},

	Disconnect {
		uuid: Uuid,
	},
}
