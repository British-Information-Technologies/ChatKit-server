use std::{io, net::SocketAddr};

use foundation::{
	networking::{read_message, write_message},
	prelude::{
		network_client_message,
		network_server_message,
		Connect,
		Connected,
		GetInfo,
		Info,
		NetworkClientMessage,
		NetworkServerMessage,
		Request,
	},
};
use tokio::{io::split, net::TcpStream};

use crate::network::{
	client_reader_connection::ClientReaderConnection,
	client_writer_connection::ClientWriterConnection,
};

pub struct NetworkConnection {
	pub(super) stream: TcpStream,
	pub(super) addr: SocketAddr,
}

impl NetworkConnection {
	pub fn new(stream: TcpStream, addr: SocketAddr) -> Self {
		Self { stream, addr }
	}

	pub async fn get_request(&mut self) -> io::Result<ServerRequest> {
		let message = NetworkServerMessage {
			message: Some(network_server_message::Message::Request(Request {})),
		};

		println!("[NetworkConnection] sending request");
		write_message(&mut self.stream, message).await.unwrap();

		println!("[NetworkConnection] waiting for response");
		let request =
			read_message::<NetworkClientMessage, TcpStream>(&mut self.stream)
				.await
				.unwrap();

		println!("[NetworkConnection] returning request");
		match request {
			NetworkClientMessage {
				message: Some(network_client_message::Message::GetInfo(GetInfo {})),
			} => Ok(ServerRequest::GetInfo),
			NetworkClientMessage {
				message:
					Some(network_client_message::Message::Connect(Connect {
						username,
						uuid,
					})),
			} => Ok(ServerRequest::Connect {
				username,
				uuid: uuid.parse().unwrap(),
				addr: self.addr,
			}),
			_ => Ok(ServerRequest::Ignore),
		}
	}

	pub async fn send_info(mut self, name: String, owner: String) {
		let message = NetworkServerMessage {
			message: Some(network_server_message::Message::GotInfo(Info {
				server_name: name,
				owner,
			})),
		};
		println!("[NetworkConnection] Sending info to client");
		write_message(&mut self.stream, message).await.unwrap();
		println!("[NetworkConnection] droping connection");
	}

	pub async fn send_connected(
		mut self,
	) -> (ClientWriterConnection, ClientReaderConnection) {
		let message = NetworkServerMessage {
			message: Some(network_server_message::Message::Connected(Connected {})),
		};

		write_message(&mut self.stream, message).await.unwrap();

		self.into()
	}
}

pub enum ServerRequest {
	GetInfo,
	Connect {
		username: String,
		uuid: uuid::Uuid,
		addr: SocketAddr,
	},
	Ignore,
}

impl From<NetworkConnection>
	for (ClientWriterConnection, ClientReaderConnection)
{
	fn from(value: NetworkConnection) -> Self {
		let (read, write) = split(value.stream);

		let writer = ClientWriterConnection::new(write, value.addr.clone());
		let reader = ClientReaderConnection::new(read, value.addr.clone());
		(writer, reader)
	}
}
