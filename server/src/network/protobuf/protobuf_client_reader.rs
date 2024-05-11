use std::{io, net::SocketAddr};

use foundation::{
	networking::protobuf::read_message,
	prelude::{
		connected_client_message,
		ConnectedClientMessage,
		Disconnect,
		GetClients,
		GetGlobalMessages,
		SendGlobalMessage,
		SendPrivateMessage,
	},
};
use tokio::{io::ReadHalf, net::TcpStream, sync::mpsc::UnboundedSender};
use uuid::Uuid;

use crate::{
	connection::connection_manager::ConnectionManagerMessage,
	network::ClientReader,
};

pub struct ProtobufClientReader {
	reader: ReadHalf<TcpStream>,
	addr: SocketAddr,
	uuid: Uuid,
}

impl ProtobufClientReader {
	pub fn new(
		reader: ReadHalf<TcpStream>,
		addr: SocketAddr,
		uuid: Uuid,
	) -> Self {
		Self { reader, addr, uuid }
	}

	// move to other one
	pub async fn get_message(&mut self) -> io::Result<ConnectedClientMessage> {
		read_message::<ConnectedClientMessage, ReadHalf<TcpStream>>(
			&mut self.reader,
		)
		.await
	}

	pub fn handle_message(
		&self,

		msg: ConnectedClientMessage,
		channel: &UnboundedSender<ConnectionManagerMessage>,
	) {
		use connected_client_message::Message;

		println!("[ProtobufClientReader:{}] got message", self.addr);

		let uuid = self.uuid;

		_ = match msg {
			ConnectedClientMessage {
				message: Some(Message::GetClients(GetClients {})),
			} => channel.send(ConnectionManagerMessage::SendClientsTo { uuid }),
			ConnectedClientMessage {
				message: Some(Message::GetGlobalMessage(GetGlobalMessages {})),
			} => channel.send(ConnectionManagerMessage::SendGlobalMessages { uuid }),
			ConnectedClientMessage {
				message:
					Some(Message::SendPrivateMessage(SendPrivateMessage {
						uuid: message_uuid,
						to,
						content,
					})),
			} => channel.send(ConnectionManagerMessage::SendPrivateMessage {
				uuid: message_uuid.parse().unwrap(),
				from: uuid,
				to: to.parse().unwrap(),
				content,
			}),
			ConnectedClientMessage {
				message: Some(Message::SendGlobalMessage(SendGlobalMessage { content })),
			} => channel.send(ConnectionManagerMessage::BroadcastGlobalMessage {
				from: uuid,
				content,
			}),
			ConnectedClientMessage {
				message: Some(Message::Disconnect(Disconnect {})),
			} => channel.send(ConnectionManagerMessage::Disconnect { uuid }),
			ConnectedClientMessage { message: None } => unimplemented!(),
		};
	}
}

impl ClientReader for ProtobufClientReader {
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
						"[ProtobufClientReader:{}] errored with '{}' disconnecting",
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
