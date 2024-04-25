use std::{io, net::SocketAddr};

use foundation::{
	networking::{read_message, write_message},
	prelude::{
		connected_server_message,
		ClientDetails,
		ConnectedClientMessage,
		ConnectedClients,
		ConnectedServerMessage,
		Disconnected,
		GlobalMessage,
		GlobalMessages,
		PrivateMessage,
	},
};
use tokio::{
	io::{split, WriteHalf},
	net::TcpStream,
};

use crate::network::{
	client_reader_connection::ClientReaderConnection,
	network_connection::NetworkConnection,
};

pub struct ClientWriterConnection {
	writer: WriteHalf<TcpStream>,
	addr: SocketAddr,
}

impl ClientWriterConnection {
	pub fn new(writer: WriteHalf<TcpStream>, addr: SocketAddr) -> Self {
		Self { writer, addr }
	}

	pub async fn send_clients(&mut self, clients: Vec<ClientDetails>) {
		let message = ConnectedServerMessage {
			message: Some(connected_server_message::Message::ConnectedClients(
				ConnectedClients { clients },
			)),
		};
		write_message(&mut self.writer, message).await.unwrap();
	}

	pub async fn send_global_messages(&mut self, messages: Vec<GlobalMessage>) {
		let message = ConnectedServerMessage {
			message: Some(connected_server_message::Message::GlobalMessages(
				GlobalMessages { messages },
			)),
		};
		write_message(&mut self.writer, message).await.unwrap();
	}

	pub async fn send_private_message(&mut self, message: PrivateMessage) {
		let message = ConnectedServerMessage {
			message: Some(connected_server_message::Message::PrivateMessage(message)),
		};
		write_message(&mut self.writer, message).await.unwrap();
	}

	pub async fn send_disconnect(&mut self) {
		let message = ConnectedServerMessage {
			message: Some(connected_server_message::Message::Disconnected(
				Disconnected {
					reason: "shutting down".into(),
				},
			)),
		};
		write_message(&mut self.writer, message).await.unwrap();
	}
}
