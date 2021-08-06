use std::sync::Arc;
use std::cmp::Ordering;

use uuid::Uuid;
use futures::lock::Mutex;
use tokio::sync::mpsc::{Sender, Receiver, channel};

use crate::network::SocketHandler;
use crate::messages::ClientMessage;
use crate::messages::ServerMessage;
use crate::prelude::StreamMessageSender;

use foundation::ClientDetails;
use foundation::messages::client::{ClientStreamIn, ClientStreamOut};

/// # Client
/// This struct represents a connected user.
///
/// ## Attributes
/// - details: store of the clients infomation.
///
/// - stream: The socket for the connected client.
/// - stream_reader: the buffered reader used to receive messages
/// - stream_writer: the buffered writer used to send messages
/// - owner: An optional reference to the owning object.
#[derive(Debug)]
pub struct Client {
	pub details: ClientDetails,

	// server send channel
	server_channel: Mutex<Sender<ServerMessage>>,

	// object channels
	tx: Sender<ClientMessage>,
	rx: Mutex<Receiver<ClientMessage>>,

	socket_sender: Arc<SocketHandler>,
}

// client funciton implmentations
impl Client {
	pub fn new(
		uuid: String,
		username: String,
		address: String,
		socket_sender: Arc<SocketHandler>,
		server_channel: Sender<ServerMessage>,
	) -> Arc<Client> {
		let (sender, receiver) = channel(1024);

		Arc::new(Client {
			details: ClientDetails {
				uuid: Uuid::parse_str(&uuid).expect("invalid id"),
				username,
				address,
        public_key: None
			},

			server_channel: Mutex::new(server_channel),
			socket_sender,

			tx: sender,
			rx: Mutex::new(receiver),

		})
	}

	pub fn start(self: &Arc<Client>) {

		let t1_client = self.clone();
		let t2_client = self.clone();

		// client stream read task
		tokio::spawn(async move {
			use ClientMessage::Disconnect;

			let client = t1_client;

			client.socket_sender.send::<ClientStreamOut>(ClientStreamOut::Connected).await.expect("error");

			loop {
				let command = client.socket_sender.recv::<ClientStreamIn>().await;
				match command {
					Ok(ClientStreamIn::Disconnect) => {
						println!("[Client {:?}]: Disconnect recieved", &client.details.uuid);
						client.send_message(Disconnect).await;
						return;
					}
					Ok(ClientStreamIn::SendMessage { to, content }) => {
						println!("[Client {:?}]: send message to: {:?}", &client.details.uuid, &to);
						let lock = client.server_channel.lock().await;
						let _ = lock.send(ServerMessage::ClientSendMessage {
							from: client.details.uuid,
							to,
							content,
						}).await;
					}
					Ok(ClientStreamIn::Update) => {
						println!("[Client {:?}]: update received", &client.details.uuid);
						let lock = client.server_channel.lock().await;
						let _ = lock.send(ServerMessage::ClientUpdate { to: client.details.uuid }).await;
					}
					_ => {
						println!("[Client {:?}]: command not found", &client.details.uuid);
						let lock = client.server_channel.lock().await;
						let _ = lock.send(ServerMessage::ClientError { to: client.details.uuid }).await;
					}
				}
			}
		});

		// client channel read thread
		tokio::spawn(async move {
			use ClientMessage::{Disconnect, Message, SendClients, Error};

			let client = t2_client;

			loop {
				let mut channel = client.rx.lock().await;

				let message = channel.recv().await.unwrap();
				drop(channel);

				println!("[Client {:?}]: {:?}", &client.details.uuid, message);
				match message {
					Disconnect => {
						let lock = client.server_channel.lock().await;
						let _ = lock.send(ServerMessage::ClientDisconnected { id: client.details.uuid }).await;
						return
					}
					Message { from, content } => 
						client.socket_sender.send::<ClientStreamOut>(
							ClientStreamOut::UserMessage { from, content }
						).await.expect("error sending message"),
					
					SendClients { clients } => {
						let client_details_vec: Vec<ClientDetails> = 
							clients.iter().map(|client| &client.details)
							.cloned().collect();

						client.socket_sender.send::<ClientStreamOut>(
							ClientStreamOut::ConnectedClients {
								clients: client_details_vec,
							}
						).await.expect("error sending message");
					},
					Error => 
						client.socket_sender.send::<ClientStreamOut>(
							ClientStreamOut::Error
						).await.expect("error sending message"),
				}
			}
		});		
	}

	pub async fn send_message(self: &Arc<Client>, msg: ClientMessage) {
		let _ = self.tx.send(msg).await;
	}
}

// MARK: - used for sorting.
impl PartialEq for Client {
	fn eq(&self, other: &Self) -> bool {
		self.details.uuid == other.details.uuid
	}
}

impl Eq for Client {}

impl PartialOrd for Client {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Client {
	fn cmp(&self, other: &Self) -> Ordering {
		self.details.uuid.cmp(&other.details.uuid)
	}
}

impl Drop for Client {
	fn drop(&mut self) {
		println!("[Client] dropped!");
	}
}
