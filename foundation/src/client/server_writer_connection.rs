use tokio::{io::WriteHalf, net::TcpStream};

pub struct ServerWriterConnection {
	writer: WriteHalf<TcpStream>,
}

impl ServerWriterConnection {
	pub(crate) fn new(writer: WriteHalf<TcpStream>) -> Self {
		Self { writer }
	}

	pub async fn request_clients(&mut self) {}
}
