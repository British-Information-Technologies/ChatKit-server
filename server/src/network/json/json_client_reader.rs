use std::{io, net::SocketAddr};

use foundation::{
	messages::client::ClientStreamIn,
	networking::json::read_message,
};
use tokio::{io::ReadHalf, net::TcpStream, sync::mpsc::UnboundedSender};
use uuid::Uuid;

use crate::{
	connection::connection_manager::ConnectionManagerMessage,
	network::ClientReader,
};

pub struct JSONClientReader {
	reader: ReadHalf<TcpStream>,
	addr: SocketAddr,
	uuid: Uuid,
}

impl JSONClientReader {
	pub fn new(
		reader: ReadHalf<TcpStream>,
		addr: SocketAddr,
		uuid: Uuid,
	) -> Self {
		Self { reader, addr, uuid }
	}

	// move to other one
	pub async fn get_message(&mut self) -> io::Result<ClientStreamIn> {
		read_message::<ReadHalf<TcpStream>, ClientStreamIn>(&mut self.reader).await
	}

	pub fn handle_message(
		&self,
		msg: ClientStreamIn,
		channel: &UnboundedSender<ConnectionManagerMessage>,
	) {
		println!("[JSONClientReader:{}] got message", self.addr);

		let uuid = self.uuid;

		_ = match msg {
			ClientStreamIn::GetClients => {
				channel.send(ConnectionManagerMessage::SendClientsTo { uuid })
			}
			ClientStreamIn::GetMessages => {
				channel.send(ConnectionManagerMessage::SendGlobalMessages { uuid })
			}
			ClientStreamIn::SendMessage { to, content } => {
				channel.send(ConnectionManagerMessage::SendPrivateMessage {
					uuid: Uuid::new_v4(),
					from: uuid,
					to,
					content,
				})
			}
			ClientStreamIn::SendGlobalMessage { content } => {
				channel.send(ConnectionManagerMessage::BroadcastGlobalMessage {
					from: uuid,
					content,
				})
			}
			ClientStreamIn::Disconnect => {
				channel.send(ConnectionManagerMessage::Disconnect { uuid })
			}
		};
	}
}

impl ClientReader for JSONClientReader {
	fn start_run(
		mut self: Box<Self>,
		uuid: Uuid,
		channel: UnboundedSender<ConnectionManagerMessage>,
	) -> tokio::task::JoinHandle<()> {
		tokio::spawn(async move {
			loop {
				let msg = self.get_message().await;

				let Ok(msg) = msg else {
					let error = msg.unwrap_err();
					println!(
						"[JSONClientReader:{}] errored with '{}' disconnecting",
						self.addr, error
					);

					_ = channel.send(ConnectionManagerMessage::Disconnected { uuid });

					return;
				};

				self.handle_message(msg, &channel);
			}
		})
	}
}
