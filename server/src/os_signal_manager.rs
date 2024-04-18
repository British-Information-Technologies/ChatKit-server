use tokio::sync::mpsc::UnboundedSender;

use crate::server_va::ServerMessages;

pub struct OSSignalManager {
	server_channel: UnboundedSender<ServerMessages>,
}

impl OSSignalManager {
	pub fn new(channel: UnboundedSender<ServerMessages>) -> Self {
		Self {
			server_channel: channel,
		}
	}

	pub async fn run(&self) {
		loop {
			println!("[OSSignalManager] waiting for ctrl+c");
			tokio::signal::ctrl_c().await.unwrap();
			println!("[OSSignalManager] ctrl+c received, closing down server");
			self
				.server_channel
				.send(ServerMessages::Exit)
				.expect("[OSSignalManager] server channel closed");
		}
	}
}
