use std::sync::Arc;
use std::cmp::Ordering;
use std::fmt::Write;

use uuid::Uuid;

use zeroize::Zeroize;

use futures::lock::Mutex;

use tokio::task;
use tokio::io::{ReadHalf, WriteHalf};
use tokio::sync::mpsc::{Sender, Receiver, channel};
use tokio::io::{AsyncBufReadExt, BufReader, AsyncWriteExt};

use crate::messages::ClientMessage;
use crate::messages::ServerMessage;

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

	stream_rx: Mutex<BufReader<ReadHalf<tokio::net::TcpStream>>>,
	stream_tx: Mutex<WriteHalf<tokio::net::TcpStream>>,
}

// client funciton implmentations
impl Client {
	pub fn new(
		uuid: String,
		username: String,
		address: String,
		stream_rx: BufReader<ReadHalf<tokio::net::TcpStream>>,
		stream_tx: WriteHalf<tokio::net::TcpStream>,
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

			tx: sender,
			rx: Mutex::new(receiver),

			stream_rx: Mutex::new(stream_rx),
			stream_tx: Mutex::new(stream_tx),
		})
	}

	pub fn start(self: &Arc<Client>) {

		let t1_client = self.clone();
		let t2_client = self.clone();

		// client stream read task
		tokio::spawn(async move {

			use ClientMessage::Disconnect;

			let client = t1_client;

			let mut lock = client.stream_tx.lock().await;
			let mut buffer = String::new();

			// tell client that is is now connected
			let _ = writeln!(buffer, "{}",
				serde_json::to_string(&ClientStreamOut::Connected).unwrap()
			);

			let _ = lock.write_all(&buffer.as_bytes());
			let _ = lock.flush().await;

			drop(lock);

			loop {
				let mut stream_reader = client.stream_rx.lock().await;
				let mut buffer = String::new();

				if let Ok(_size) = stream_reader.read_line(&mut buffer).await {

					let command = serde_json::from_str::<ClientStreamIn>(buffer.as_str());
					println!("[Client {:?}]: recieved {}", client.details.uuid, &buffer);

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
							});
						}
						Ok(ClientStreamIn::Update) => {
							let lock = client.server_channel.lock().await;
							let _ = lock.send(ServerMessage::ClientUpdate { to: client.details.uuid });
						}
						_ => println!("[Client {:?}]: command not found", &client.details.uuid),
					}
					buffer.zeroize();
				}
				println!("[Client {:?}] exited thread 1", &client.details.uuid);
			}
		});

		// client channel read thread
		tokio::spawn(async move {
			use ClientMessage::{Disconnect, Message, SendClients};

			let client = t2_client;

			loop {
				let mut channel = client.rx.lock().await;
				let mut buffer = String::new();

				let message = channel.recv().await.unwrap();
				drop(channel);

				println!("[Client {:?}]: {:?}", &client.details.uuid, message);
				match message {
					Disconnect => {
						let lock = client.server_channel.lock().await;
						let _ = lock.send(ServerMessage::ClientDisconnected { id: client.details.uuid }).await;
						return
					}
					Message { from, content } => {
						let msg = ClientStreamOut::UserMessage { from, content };
						let _ = writeln!(buffer, "{}", serde_json::to_string(&msg).unwrap());

						let mut stream = client.stream_tx.lock().await;

						let _ = stream.write_all(&buffer.as_bytes());
						let _ = stream.flush().await;

						drop(stream);
					}
					SendClients { clients } => {
						let client_details_vec: Vec<ClientDetails> = clients
							.iter()
							.map(|client| &client.details)
							.cloned()
							.collect();

						let msg = ClientStreamOut::ConnectedClients {
							clients: client_details_vec,
						};

						let _ = writeln!(buffer, "{}", serde_json::to_string(&msg).unwrap());

						let mut stream = client.stream_tx.lock().await;


						let _ = stream.write_all(&buffer.as_bytes());
						let _ = stream.flush().await;
					}
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
