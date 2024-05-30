use foundation::prelude::{ClientDetails, GlobalMessage, PrivateMessage};
use tokio::{sync::mpsc::UnboundedSender, task::JoinHandle};
use uuid::Uuid;

use crate::{
	connection::{
		client_info::ClientInfo,
		connection_manager::ConnectionManagerMessage,
	},
	network::{ClientWriter, NetworkConnection},
};

pub struct ClientThread {
	read_task: JoinHandle<()>,
	writer: Box<dyn ClientWriter>,
}

impl ClientThread {
	pub async fn new_run(
		uuid: Uuid,
		conn: Box<dyn NetworkConnection>,
		connection_manager_sender: UnboundedSender<ConnectionManagerMessage>,
	) -> Self {
		println!("[ClientThread] creating thread");
		let (writer, reader) = conn.send_connected(uuid).await;

		println!("[ClientThread] creating tasks");
		ClientThread {
			read_task: reader.start_run(uuid, connection_manager_sender.clone()),
			writer,
		}
	}

	pub async fn send_clients(&mut self, clients: Vec<ClientDetails>) {
		self.writer.send_clients(clients).await
	}

	pub async fn send_client_joined(&mut self, details: ClientDetails) {
		self.writer.send_client_joined(details).await;
	}
	pub async fn send_client_left(&mut self, uuid: Uuid) {
		self.writer.send_client_left(uuid).await
	}

	// todo: link this in with message storage
	pub(crate) async fn send_global_message(&mut self, message: GlobalMessage) {
		self.writer.send_global_message(message).await;
	}

	pub(crate) async fn send_global_messages(
		&mut self,
		messages: Vec<GlobalMessage>,
	) {
		self.writer.send_global_messages(messages).await;
	}

	pub(crate) async fn send_disconnected(&mut self) {
		self.writer.send_disconnect().await
	}

	pub(crate) async fn send_private_message(
		&mut self,
		from: Uuid,
		uuid: Uuid,
		content: String,
	) {
		self
			.writer
			.send_private_message(PrivateMessage {
				uuid: uuid.to_string(),
				from: from.to_string(),
				content,
			})
			.await;
	}
}

impl Drop for ClientThread {
	fn drop(&mut self) {
		self.read_task.abort();
	}
}

pub enum ClientMessage {
	SendClients(Vec<ClientInfo>),
	SendGlobalMessages(Vec<GlobalMessage>),
}
