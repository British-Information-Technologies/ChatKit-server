use std::net::SocketAddr;

use tokio::net::TcpStream;

use crate::network_connection::NetworkConnection;

struct ClientConnection {
	stream: TcpStream,
	_addr: SocketAddr,
}

impl From<NetworkConnection> for ClientConnection {
	fn from(value: NetworkConnection) -> Self {
		Self {
			stream: value.
		}
	}
}

impl ClientConnection {}
