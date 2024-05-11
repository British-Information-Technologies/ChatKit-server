use std::{io, net::SocketAddr};

use foundation::{
	messages::network::{NetworkSockIn, NetworkSockOut},
	networking::json::{read_message, write_message},
};
use tokio::{io::split, net::TcpStream};
use uuid::Uuid;

use crate::network::{
	json::{
		json_client_reader::JSONClientReader,
		json_client_writer::JSONClientWriter,
	},
	ClientReader,
	ClientWriter,
	NetworkConnection,
	ServerRequest,
};

pub struct JSONNetworkConnection {
	pub(super) stream: TcpStream,
	pub(super) addr: SocketAddr,
}

impl JSONNetworkConnection {
	pub fn new(stream: TcpStream, addr: SocketAddr) -> Self {
		Self { stream, addr }
	}
}

#[async_trait::async_trait]
impl NetworkConnection for JSONNetworkConnection {
	async fn get_request(&mut self) -> io::Result<ServerRequest> {
		println!("[JSONNetworkConnection] sending request");

		write_message(&mut self.stream, NetworkSockOut::Request).await;

		println!("[JSONNetworkConnection] waiting for response");

		let request =
			read_message::<TcpStream, NetworkSockIn>(&mut self.stream).await?;

		println!("[JSONNetworkConnection] returning request");

		match request {
			NetworkSockIn::Info => Ok(ServerRequest::GetInfo),
			NetworkSockIn::Connect {
				uuid,
				username,
				address: _,
			} => Ok(ServerRequest::Connect {
				username,
				uuid,
				addr: self.addr,
			}),
			// _ => Ok(ServerRequest::Ignore),
		}
	}

	async fn send_info(mut self: Box<Self>, name: String, owner: String) {
		println!("[JSONNetworkConnection] Sending info to client");
		write_message(
			&mut self.stream,
			NetworkSockOut::GotInfo {
				server_name: name,
				server_owner: owner,
			},
		)
		.await;

		println!("[JSONNetworkConnection] droping connection");
	}

	async fn send_connected(
		mut self: Box<Self>,
		uuid: Uuid,
	) -> (Box<dyn ClientWriter>, Box<dyn ClientReader>) {
		write_message(&mut self.stream, NetworkSockOut::Connected).await;

		let (read, write) = split(self.stream);

		let writer = Box::new(JSONClientWriter::new(write, self.addr, uuid));
		let reader = Box::new(JSONClientReader::new(read, self.addr, uuid));
		(writer, reader)
	}
}
