use std::{io, net::SocketAddr};

use protocol::prelude::{
	network_client_message,
	network_server_message,
	Connect,
	GetInfo,
	Info,
	NetworkClientMessage,
	NetworkServerMessage,
	Request,
};
use tokio::{io::split, net::TcpStream};
use uuid::Uuid;

use crate::{
	client::{
		server_reader_connection::ServerReaderConnection,
		server_writer_connection::ServerWriterConnection,
	},
	networking::protobuf::{read_message, write_message},
};

/// # NetworkConnection
/// encapsulates the state of the network connection
/// will connect to a server and ensure it is usinghte protobuf protocol
///
/// you can then either get info or connect to the server
pub struct NetworkConnection {
	pub(super) stream: TcpStream,
}

impl NetworkConnection {
	pub async fn connect(address: SocketAddr) -> io::Result<Self> {
		let mut stream = TcpStream::connect(address).await.unwrap();

		let msg =
			read_message::<NetworkServerMessage, TcpStream>(&mut stream).await?;

		let NetworkServerMessage {
			message: Some(network_server_message::Message::Request(Request {})),
		} = msg
		else {
			return Err(io::Error::new(
				io::ErrorKind::InvalidData,
				"Received invalid start message from server",
			));
		};

		Ok(Self { stream })
	}

	/// Will consume the connection, and fetch the servers info.
	pub async fn send_get_info(mut self) -> io::Result<Info> {
		_ = write_message(
			&mut self.stream,
			NetworkClientMessage {
				message: Some(network_client_message::Message::GetInfo(GetInfo {})),
			},
		)
		.await;

		let message =
			read_message::<NetworkServerMessage, TcpStream>(&mut self.stream).await?;

		let NetworkServerMessage {
			message: Some(network_server_message::Message::GotInfo(msg)),
		} = message
		else {
			return Err(io::Error::new(
				io::ErrorKind::InvalidData,
				"sent for info got different message back",
			));
		};

		Ok(msg)
	}

	/// consumes this struct and returns a tuple of the sernding and receiving ahlfs of teh connected conneciton
	pub async fn send_connect(
		mut self,
		uuid: Uuid,
		username: String,
	) -> io::Result<(ServerWriterConnection, ServerReaderConnection)> {
		_ = write_message(
			&mut self.stream,
			NetworkClientMessage {
				message: Some(network_client_message::Message::Connect(Connect {
					username,
					uuid: uuid.to_string(),
				})),
			},
		)
		.await;

		let message =
			read_message::<NetworkServerMessage, TcpStream>(&mut self.stream).await?;

		let NetworkServerMessage {
			message: Some(network_server_message::Message::Connected(_)),
		} = message
		else {
			return Err(io::Error::new(
				io::ErrorKind::InvalidData,
				"sent connect got different message back or failed to connect",
			));
		};

		Ok(self.into())
	}
}

impl From<NetworkConnection>
	for (ServerWriterConnection, ServerReaderConnection)
{
	fn from(value: NetworkConnection) -> Self {
		let (read_half, write_half) = split(value.stream);
		(
			ServerWriterConnection::new(write_half),
			ServerReaderConnection::new(read_half),
		)
	}
}
