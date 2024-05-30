use std::io;

use protocol::prelude::ConnectedServerMessage;
use tokio::{io::ReadHalf, net::TcpStream};

use crate::networking::protobuf::read_message;

pub struct ServerReaderConnection {
	reader: ReadHalf<TcpStream>,
}

impl ServerReaderConnection {
	pub(crate) fn new(read_half: ReadHalf<TcpStream>) -> Self {
		Self { reader: read_half }
	}

	// move to other one
	pub async fn get_message(&mut self) -> io::Result<ConnectedServerMessage> {
		let message = read_message::<ConnectedServerMessage, ReadHalf<TcpStream>>(
			&mut self.reader,
		)
		.await
		.unwrap();
		Ok(message)
	}
}
