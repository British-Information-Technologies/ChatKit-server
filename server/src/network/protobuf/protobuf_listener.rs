use async_trait::async_trait;
use tokio::{
	net::TcpListener,
	select,
	sync::mpsc::UnboundedSender,
	task::JoinHandle,
};

use crate::{
	network::{ConnectionType, NetworkListener},
	server_va::ServerMessages,
};

/// # Listener Manager
/// This stores and awaits for connections from listeners.
/// When a connection is received, it is passed to the server
pub struct ProtobufListener {
	protobuf_listener: TcpListener,
	sender: UnboundedSender<ServerMessages>,
}

#[async_trait]
impl NetworkListener for ProtobufListener {
	/// Binds listeners and stores them in the ListenerManager
	async fn new(channel: UnboundedSender<ServerMessages>) -> Self {
		println!("[ProtobufListener] setting up listeners");
		let protobuf_listener = TcpListener::bind("0.0.0.0:6500")
			.await
			.expect("[ProtobufListener] failed to bind to 0.0.0.0:6500");

		Self {
			protobuf_listener,
			sender: channel,
		}
	}

	async fn run(&self) {
		loop {
			println!("[ProtobufListener] waiting for connection");
			let accept_protobuf = self.protobuf_listener.accept();

			let msg = select! {
				Ok((stream, addr)) = accept_protobuf => {
					println!("[ProtobufListener] accepted connection");
					ServerMessages::NewConnection(ConnectionType::ProtobufConnection(stream, addr))
				}
			};
			println!("[ProtobufListener] passing message to server");
			self.sender.send(msg).unwrap();
		}
	}

	fn start_run(sender: UnboundedSender<ServerMessages>) -> JoinHandle<()> {
		tokio::spawn(async move {
			ProtobufListener::new(sender).await.run().await;
		})
	}
}
