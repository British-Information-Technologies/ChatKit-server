use std::net::SocketAddr;

use async_trait::async_trait;
use chrono::Local;
use foundation::{
	messages::client::ClientStreamOut,
	models::message::Message,
	networking::json::write_message,
	prelude::{GlobalMessage, PrivateMessage},
	ClientDetails,
};
use tokio::{io::WriteHalf, net::TcpStream};
use uuid::Uuid;

use crate::network::ClientWriter;

#[allow(dead_code)]
pub struct JSONClientWriter {
	writer: WriteHalf<TcpStream>,
	addr: SocketAddr,
	uuid: Uuid,
}

impl JSONClientWriter {
	pub fn new(
		writer: WriteHalf<TcpStream>,
		addr: SocketAddr,
		uuid: Uuid,
	) -> Self {
		Self { writer, addr, uuid }
	}
}

#[async_trait]
impl ClientWriter for JSONClientWriter {
	async fn send_clients(
		&mut self,
		clients: Vec<foundation::prelude::ClientDetails>,
	) {
		let message = ClientStreamOut::ConnectedClients {
			clients: clients
				.into_iter()
				.map(|c| ClientDetails {
					uuid: c.uuid.parse().unwrap(),
					username: c.name,
					address: c.address,
					public_key: None,
				})
				.collect(),
		};
		println!("[JSONClientWriter:{}] sending clients", self.addr);
		write_message(&mut self.writer, message).await;
	}

	async fn send_client_joined(
		&mut self,
		details: foundation::prelude::ClientDetails,
	) {
		let message = ClientStreamOut::ClientConnected {
			id: details.uuid.parse().unwrap(),
			username: details.name,
		};
		println!(
			"[ProtobufClientWriter:{}] sending client connected message",
			self.addr
		);
		write_message(&mut self.writer, message).await;
	}

	async fn send_client_left(&mut self, uuid: Uuid) {
		let message = ClientStreamOut::ClientRemoved { id: uuid };
		println!(
			"[ProtobufClientWriter:{}] sending client connected message",
			self.addr
		);
		write_message(&mut self.writer, message).await;
	}

	async fn send_global_messages(&mut self, messages: Vec<GlobalMessage>) {
		let message = ClientStreamOut::GlobalChatMessages {
			messages: messages
				.into_iter()
				.map(|m| Message {
					id: m.uuid.parse().unwrap(),
					from: m.from.parse().unwrap(),
					content: m.content,
					time: Local::now(),
				})
				.collect(),
		};
		println!("[JSONClientWriter:{}] sending global messages", self.addr);
		write_message(&mut self.writer, message).await;
	}

	async fn send_private_message(&mut self, message: PrivateMessage) {
		let message = ClientStreamOut::UserMessage {
			from: message.from.parse().unwrap(),
			content: message.content,
		};
		println!("[JSONClientWriter:{}] sending private message", self.addr);
		write_message(&mut self.writer, message).await;
	}

	async fn send_global_message(&mut self, message: GlobalMessage) {
		let message = ClientStreamOut::GlobalMessage {
			from: message.from.parse().unwrap(),
			content: message.content,
		};
		write_message(&mut self.writer, message).await;
	}

	async fn send_disconnect(&mut self) {
		let message = ClientStreamOut::Disconnected;
		println!("[JSONClientWriter:{}] sending disconnect", self.addr);
		write_message(&mut self.writer, message).await;
	}
}
