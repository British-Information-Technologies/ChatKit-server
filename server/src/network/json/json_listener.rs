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
pub struct JSONListener {
	listener: TcpListener,
	sender: UnboundedSender<ServerMessages>,
}

#[async_trait]
impl NetworkListener for JSONListener {
	/// Binds listeners and stores them in the ListenerManager
	async fn new(sender: UnboundedSender<ServerMessages>) -> Self {
		let address = "0.0.0.0:5600";

		println!("[JSONListener] setting up listeners");
		let listener = TcpListener::bind(address)
			.await
			.expect("[JSONListener] failed to bind to 0.0.0.0:5600");

		Self { listener, sender }
	}

	async fn run(&self) {
		loop {
			println!("[JSONListener] waiting for connection");
			let accept_protobuf = self.listener.accept();

			let msg = select! {
				Ok((stream, addr)) = accept_protobuf => {
					println!("[JSONListener] accepted connection");
					ServerMessages::NewConnection(ConnectionType::JsonConnection(stream, addr))
				}
			};
			println!("[JSONListener] passing message to server");
			self.sender.send(msg).unwrap();
		}
	}

	fn start_run(sender: UnboundedSender<ServerMessages>) -> JoinHandle<()> {
		tokio::spawn(async move {
			JSONListener::new(sender).await.run().await;
		})
	}
}
