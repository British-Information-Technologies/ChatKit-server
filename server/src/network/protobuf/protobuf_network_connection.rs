use std::{io, net::SocketAddr};

use async_trait::async_trait;
use foundation::{
	networking::protobuf::{read_message, write_message},
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
use uuid::Uuid;

use crate::network::{
	protobuf::{
		protobuf_client_reader::ProtobufClientReader,
		protobuf_client_writer::ProtobufClientWriter,
	},
	ClientReader,
	ClientWriter,
	NetworkConnection,
	ServerRequest,
};

pub struct ProtobufNetworkConnection {
	pub(super) stream: TcpStream,
	pub(super) addr: SocketAddr,
}

impl ProtobufNetworkConnection {
	pub fn new(stream: TcpStream, addr: SocketAddr) -> Self {
		Self { stream, addr }
	}

	pub async fn get_request(&mut self) -> io::Result<ServerRequest> {
		let message = NetworkServerMessage {
			message: Some(network_server_message::Message::Request(Request {})),
		};

		println!("[ProtobufNetworkConnection] sending request");
		write_message(&mut self.stream, message).await.unwrap();

		println!("[ProtobufNetworkConnection] waiting for response");
		let request =
			read_message::<NetworkClientMessage, TcpStream>(&mut self.stream)
				.await
				.unwrap();

		println!("[ProtobufNetworkConnection] returning request");
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
		println!("[ProtobufNetworkConnection] Sending info to client");
		write_message(&mut self.stream, message).await.unwrap();
		println!("[ProtobufNetworkConnection] droping connection");
	}

	pub async fn send_connected(
		mut self,
		uuid: Uuid,
	) -> (ProtobufClientWriter, ProtobufClientReader) {
		let message = NetworkServerMessage {
			message: Some(network_server_message::Message::Connected(Connected {})),
		};

		write_message(&mut self.stream, message).await.unwrap();

		self.into(uuid)
	}

	fn into(self, uuid: Uuid) -> (ProtobufClientWriter, ProtobufClientReader) {
		let (read, write) = split(self.stream);

		let writer = ProtobufClientWriter::new(write, self.addr, uuid);
		let reader = ProtobufClientReader::new(read, self.addr, uuid);
		(writer, reader)
	}
}

#[async_trait]
impl NetworkConnection for ProtobufNetworkConnection {
	async fn get_request(&mut self) -> io::Result<ServerRequest> {
		let message = NetworkServerMessage {
			message: Some(network_server_message::Message::Request(Request {})),
		};

		println!("[ProtobufNetworkConnection] sending request");
		write_message(&mut self.stream, message).await.unwrap();

		println!("[ProtobufNetworkConnection] waiting for response");
		let request =
			read_message::<NetworkClientMessage, TcpStream>(&mut self.stream)
				.await
				.unwrap();

		println!("[ProtobufNetworkConnection] returning request");
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

	async fn send_info(mut self: Box<Self>, name: String, owner: String) {
		let message = NetworkServerMessage {
			message: Some(network_server_message::Message::GotInfo(Info {
				server_name: name,
				owner,
			})),
		};
		println!("[ProtobufNetworkConnection] Sending info to client");
		write_message(&mut self.stream, message).await.unwrap();
		println!("[ProtobufNetworkConnection] droping connection");
	}

	async fn send_connected(
		mut self: Box<Self>,
		uuid: Uuid,
	) -> (Box<dyn ClientWriter>, Box<dyn ClientReader>) {
		let message = NetworkServerMessage {
			message: Some(network_server_message::Message::Connected(Connected {})),
		};

		write_message(&mut self.stream, message).await.unwrap();

		let (read, write) = split(self.stream);

		let writer = Box::new(ProtobufClientWriter::new(write, self.addr, uuid));
		let reader = Box::new(ProtobufClientReader::new(read, self.addr, uuid));
		(writer, reader)
	}
}
