use foundation::prelude::{
	connected_client_message,
	ClientDetails,
	ConnectedClientMessage,
	Disconnect,
	GetClients,
	GetGlobalMessages,
	SendGlobalMessage,
	SendPrivateMessage,
};
use tokio::{
	sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
	task::JoinHandle,
};
use uuid::Uuid;

use crate::{
	connection::{
		client_info::ClientInfo,
		connection_manager::ConnectionManagerMessage,
	},
	network::{
		client_reader_connection::ClientReaderConnection,
		client_writer_connection::ClientWriterConnection,
		network_connection::NetworkConnection,
	},
};

pub struct ClientThread {
	read_task: JoinHandle<()>,
	write_task: JoinHandle<()>,
	sender: UnboundedSender<ClientMessage>,
}

impl ClientThread {
	pub async fn new_run(
		uuid: Uuid,
		conn: NetworkConnection,
		connection_manager_sender: UnboundedSender<ConnectionManagerMessage>,
	) -> Self {
		let (writer, reader) = conn.send_connected().await;
		let (tx, rx) = unbounded_channel();

		Self {
			read_task: tokio::spawn(Self::run_read(
				uuid,
				reader,
				connection_manager_sender,
			)),
			write_task: tokio::spawn(Self::run_write(uuid, writer, rx)),
			sender: tx,
		}
	}

	async fn run_read(
		uuid: Uuid,
		mut reader: ClientReaderConnection,
		channel: UnboundedSender<ConnectionManagerMessage>,
	) {
		use connected_client_message::Message;

		loop {
			println!("[ClientThread:run_read:{}]", uuid);
			let msg = reader.get_message().await;

			match msg {
				Ok(ConnectedClientMessage {
					message: Some(Message::GetClients(GetClients {})),
				}) => channel.send(ConnectionManagerMessage::SendClientsTo { uuid }),
				Ok(ConnectedClientMessage {
					message: Some(Message::GetGlobalMessage(GetGlobalMessages {})),
				}) => {
					channel.send(ConnectionManagerMessage::SendGlobalMessagesTo { uuid })
				}
				Ok(ConnectedClientMessage {
					message:
						Some(Message::SendPrivateMessage(SendPrivateMessage {
							uuid: message_uuid,
							to,
							content,
						})),
				}) => channel.send(ConnectionManagerMessage::SendPrivateMessage {
					uuid: message_uuid,
					from: uuid,
					to: to.parse().unwrap(),
					content,
				}),
				Ok(ConnectedClientMessage {
					message:
						Some(Message::SendGlobalMessage(SendGlobalMessage { content })),
				}) => channel.send(ConnectionManagerMessage::BroadcastGlobalMessage {
					from: uuid,
					content,
				}),
				Ok(ConnectedClientMessage {
					message: Some(Message::Disconnect(Disconnect {})),
				}) => channel.send(ConnectionManagerMessage::Disconnect { uuid }),
				Ok(ConnectedClientMessage { message: None }) => unimplemented!(),

				Err(_) => todo!(),
			};

			break;
		}
	}

	async fn run_write(
		uuid: Uuid,
		mut conn: ClientWriterConnection,
		mut receiver: UnboundedReceiver<ClientMessage>,
	) {
		loop {
			let msg = receiver.recv().await;

			match msg {
				Some(ClientMessage::SendClients(clients)) => {
					let clients = clients
						.into_iter()
						.map(|c| ClientDetails {
							uuid: c.get_uuid().to_string(),
							name: c.get_username(),
							address: c.get_addr().to_string(),
						})
						.collect();
					conn.send_clients(clients).await
				}
				None => {}
			};
		}
	}
}

pub enum ClientMessage {
	SendClients(Vec<ClientInfo>),
}
