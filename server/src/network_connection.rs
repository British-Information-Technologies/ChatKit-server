use std::{io, net::SocketAddr};

use foundation::{
	networking::{read_message, write_message},
	prelude::{
		network_client_message,
		network_server_message,
		Connect,
		GetInfo,
		Info,
		NetworkClientMessage,
		NetworkServerMessage,
		Request,
	},
};
use tokio::net::TcpStream;

pub struct NetworkConnection {
	stream: TcpStream,
	addr: SocketAddr,
}

impl NetworkConnection {
	pub fn new(stream: TcpStream, addr: SocketAddr) -> Self {
		Self { stream, addr }
	}

	pub async fn get_request(&mut self) -> io::Result<ServerRequest> {
		let message = NetworkServerMessage {
			message: Some(network_server_message::Message::Request(Request {
				a: true,
			})),
		};

		println!("[NetworkConnection] made message {:?}", message);
		write_message(&mut self.stream, message).await.unwrap();

		let request = read_message::<NetworkClientMessage>(&mut self.stream)
			.await
			.unwrap();

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

		write_message(&mut self.stream, message).await.unwrap();
	}
}

pub enum ServerRequest {
	GetInfo,
	Connect { username: String, uuid: uuid::Uuid },
	Ignore,
}
