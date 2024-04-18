use std::net::SocketAddr;

use tokio::{
	net::{TcpListener, TcpStream},
	select,
	sync::mpsc::UnboundedSender,
};

use crate::server_va::ServerMessages;

/// # Listener Manager
/// This stores and awaits for connections from listeners.
/// When a connection is received, it is passed to the server
pub struct ListenerManager {
	protobuf_listener: TcpListener,
	sender: UnboundedSender<ServerMessages>,
}

impl ListenerManager {
	/// Binds listeners and stores them in the ListenerManager
	pub async fn new(channel: UnboundedSender<ServerMessages>) -> Self {
		let protobuf_listener = TcpListener::bind("0.0.0.0:6500")
			.await
			.expect("[ListenerManager] failed to bind to 0.0.0.0:6500");

		Self {
			protobuf_listener,
			sender: channel,
		}
	}

	pub async fn run(&self) {
		loop {
			let accept_protobuf = self.protobuf_listener.accept();

			let msg = select! {
				Ok((stream, addr)) = accept_protobuf => {
					println!("[ListenerManager] accepted connection");
					ServerMessages::NewConnection(ConnectionType::ProtobufConnection(stream, addr))
				}
			};
			println!("[ListenerManager] passing message to server");
			self.sender.send(msg).unwrap();
			println!("[ListenerManager] looping to accept new");
		}
	}
}

pub enum ConnectionType {
	ProtobufConnection(TcpStream, SocketAddr),
}
