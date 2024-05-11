use std::net::SocketAddr;

use async_trait::async_trait;
use foundation::{
	networking::protobuf::write_message,
	prelude::{
		connected_server_message,
		ClientConnected,
		ClientDetails,
		ClientDisconnected,
		ConnectedClients,
		ConnectedServerMessage,
		Disconnected,
		GlobalMessage,
		GlobalMessages,
		PrivateMessage,
	},
};
use tokio::{io::WriteHalf, net::TcpStream};
use uuid::Uuid;

use crate::network::ClientWriter;

#[allow(dead_code)]
pub struct ProtobufClientWriter {
	writer: WriteHalf<TcpStream>,
	addr: SocketAddr,
	uuid: Uuid,
}

impl ProtobufClientWriter {
	pub fn new(
		writer: WriteHalf<TcpStream>,
		addr: SocketAddr,
		uuid: Uuid,
	) -> Self {
		Self { writer, addr, uuid }
	}

	#[deprecated]
	pub async fn send_clients(&mut self, clients: Vec<ClientDetails>) {
		let message = ConnectedServerMessage {
			message: Some(connected_server_message::Message::ConnectedClients(
				ConnectedClients { clients },
			)),
		};
		println!("[ProtobufClientWriter:{}] sending clients", self.addr);
		write_message(&mut self.writer, message).await.unwrap();
	}

	#[deprecated]
	pub async fn send_global_messages(&mut self, messages: Vec<GlobalMessage>) {
		let message = ConnectedServerMessage {
			message: Some(connected_server_message::Message::GlobalMessages(
				GlobalMessages { messages },
			)),
		};
		println!(
			"[ProtobufClientWriter:{}] sending global messages",
			self.addr
		);
		write_message(&mut self.writer, message).await.unwrap();
	}

	#[deprecated]
	pub async fn send_private_message(&mut self, message: PrivateMessage) {
		let message = ConnectedServerMessage {
			message: Some(connected_server_message::Message::PrivateMessage(message)),
		};
		println!(
			"[ProtobufClientWriter:{}] sending private message",
			self.addr
		);
		write_message(&mut self.writer, message).await.unwrap();
	}
	#[deprecated]
	pub async fn send_disconnect(&mut self) {
		let message = ConnectedServerMessage {
			message: Some(connected_server_message::Message::Disconnected(
				Disconnected {
					reason: "shutting down".into(),
				},
			)),
		};
		println!("[ProtobufClientWriter:{}] sending disconnect", self.addr);
		write_message(&mut self.writer, message).await.unwrap();
	}
}

#[async_trait]
impl ClientWriter for ProtobufClientWriter {
	async fn send_clients(&mut self, clients: Vec<ClientDetails>) {
		let message = ConnectedServerMessage {
			message: Some(connected_server_message::Message::ConnectedClients(
				ConnectedClients { clients },
			)),
		};
		println!("[ProtobufClientWriter:{}] sending clients", self.addr);
		write_message(&mut self.writer, message).await.unwrap();
	}

	async fn send_client_joined(&mut self, details: ClientDetails) {
		let message = ConnectedServerMessage {
			message: Some(connected_server_message::Message::ClientConnected(
				ClientConnected {
					details: Some(details),
				},
			)),
		};
		println!(
			"[ProtobufClientWriter:{}] sending client connected message",
			self.addr
		);
		write_message(&mut self.writer, message).await.unwrap();
	}

	async fn send_client_left(&mut self, uuid: Uuid) {
		let message = ConnectedServerMessage {
			message: Some(connected_server_message::Message::ClientDisconnected(
				ClientDisconnected {
					uuid: uuid.to_string(),
				},
			)),
		};
		println!(
			"[ProtobufClientWriter:{}] sending client connected message",
			self.addr
		);
		write_message(&mut self.writer, message).await.unwrap();
	}

	async fn send_global_messages(&mut self, messages: Vec<GlobalMessage>) {
		let message = ConnectedServerMessage {
			message: Some(connected_server_message::Message::GlobalMessages(
				GlobalMessages { messages },
			)),
		};
		println!(
			"[ProtobufClientWriter:{}] sending global messages",
			self.addr
		);
		write_message(&mut self.writer, message).await.unwrap();
	}

	async fn send_global_message(&mut self, message: GlobalMessage) {
		let message = ConnectedServerMessage {
			message: Some(connected_server_message::Message::GlobalMessage(message)),
		};
		println!("[ProtobufClientWriter:{}] sending disconnect", self.addr);
		write_message(&mut self.writer, message).await.unwrap();
	}

	async fn send_private_message(&mut self, message: PrivateMessage) {
		let message = ConnectedServerMessage {
			message: Some(connected_server_message::Message::PrivateMessage(message)),
		};
		println!(
			"[ProtobufClientWriter:{}] sending private message",
			self.addr
		);
		write_message(&mut self.writer, message).await.unwrap();
	}

	async fn send_disconnect(&mut self) {
		let message = ConnectedServerMessage {
			message: Some(connected_server_message::Message::Disconnected(
				Disconnected {
					reason: "shutting down".into(),
				},
			)),
		};
		println!("[ProtobufClientWriter:{}] sending disconnect", self.addr);
		write_message(&mut self.writer, message).await.unwrap();
	}
}
