use std::{io, net::SocketAddr};

use foundation::{networking::read_message, prelude::ConnectedClientMessage};
use tokio::{io::ReadHalf, net::TcpStream};

pub struct ClientReaderConnection {
	reader: ReadHalf<TcpStream>,
	_addr: SocketAddr,
}

impl ClientReaderConnection {
	pub fn new(reader: ReadHalf<TcpStream>, addr: SocketAddr) -> Self {
		Self {
			reader: todo!(),
			_addr: todo!(),
		}
	}

	// move to other one
	pub async fn get_message(&mut self) -> io::Result<ConnectedClientMessage> {
		let message = read_message::<ConnectedClientMessage, ReadHalf<TcpStream>>(
			&mut self.reader,
		)
		.await
		.unwrap();
		Ok(message)
	}
}
