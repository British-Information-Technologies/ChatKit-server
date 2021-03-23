use crate::messages::ClientMessage;
use crate::messages::ServerMessage;
use std::cmp::Ordering;
use std::io::BufRead;
use std::io::{BufReader, BufWriter};
use std::mem::replace;
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::Mutex;

use crossbeam_channel::{unbounded, Receiver, Sender};
use serde::Serialize;
use uuid::Uuid;

use foundation::messages::client::ClientStreamIn;
use foundation::prelude::{ICooperative, IMessagable};

/// # Client
/// This struct represents a connected user.
///
/// ## Attrubutes
/// - uuid: The id of the connected user.
/// - username: The username of the connected user.
/// - address: The the address of the connected client.
///
/// - stream: The socket for the connected client.
/// - owner: An optional reference to the owning object.
#[derive(Debug, Serialize)]
pub struct Client {
	pub uuid: Uuid,
	username: String,
	address: String,

	// non serializable
	#[serde(skip)]
	server_channel: Mutex<Option<Sender<ServerMessage>>>,

	#[serde(skip)]
	input: Sender<ClientMessage>,

	#[serde(skip)]
	output: Receiver<ClientMessage>,

	#[serde(skip)]
	stream: Mutex<Option<TcpStream>>,

	#[serde(skip)]
	stream_reader: Mutex<Option<BufReader<TcpStream>>>,

	#[serde(skip)]
	stream_writer: Mutex<Option<BufWriter<TcpStream>>>,
}

// client funciton implmentations
impl Client {
	pub fn new(
		uuid: String,
		username: String,
		address: String,
		stream: TcpStream,
		server_channel: Sender<ServerMessage>,
	) -> Arc<Client> {
		let (sender, receiver) = unbounded();

		let out_stream = stream.try_clone().unwrap();
		let in_stream = stream.try_clone().unwrap();

		Arc::new(Client {
			username,
			uuid: Uuid::parse_str(&uuid).expect("invalid id"),
			address,

			server_channel: Mutex::new(Some(server_channel)),

			input: sender,
			output: receiver,

			stream: Mutex::new(Some(stream)),

			stream_reader: Mutex::new(Some(BufReader::new(in_stream))),
			stream_writer: Mutex::new(Some(BufWriter::new(out_stream))),
		})
	}

	// MARK: - removeable
	pub fn send(&self, _bytes: Vec<u8>) -> Result<(), &str> {
		todo!()
	}
	pub fn recv(&self) -> Option<Vec<u8>> {
		todo!()
	}
	// Mark: end -
}

impl IMessagable<ClientMessage, Sender<ServerMessage>> for Client {
	fn send_message(&self, msg: ClientMessage) {
		self.input
			.send(msg)
			.expect("failed to send message to client.");
	}
	fn set_sender(&self, sender: Sender<ServerMessage>) {
		let mut server_lock = self.server_channel.lock().unwrap();
		let _ = replace(&mut *server_lock, Some(sender));
	}
}

// cooperative multitasking implementation
impl ICooperative for Client {
	fn tick(&self) {
		println!("[client]: Tick!");
		{
			// aquire locks (so value isn't dropped)
			let mut reader_lock = self.stream_reader.lock().unwrap();
			let mut writer_lock = self.stream_writer.lock().unwrap();

			// aquiring mutable buffers
			let reader = reader_lock.as_mut().unwrap();
			let _writer = writer_lock.as_mut().unwrap();

			// create buffer
			let mut buffer = String::new();

			// loop over all lines that have been sent.
			while let Ok(_size) = reader.read_line(&mut buffer) {
				let command =
					serde_json::from_str::<ClientStreamIn>(buffer.as_str())
						.unwrap();

				match command {
					ClientStreamIn::Disconnect => println!("got Disconnect"),
					_ => println!("New command found"),
				}
			}
		}

		{
			for message in self.output.iter() {
				use ClientMessage::Disconnect;
				match message {
					Disconnect => {
						let lock = self.server_channel.lock().unwrap();

						if let Some(sender) = lock.as_ref() {
							sender
								.send(ServerMessage::ClientDisconnected(
									self.uuid,
								))
								.unwrap();
						}
					}
					_ => println!("command not implemneted yet"),
				}
			}
		}

		// handle incomming messages
	}
}

// default value implementation
impl Default for Client {
	fn default() -> Self {
		let (sender, reciever) = unbounded();
		Client {
			username: "generic_client".to_string(),
			uuid: Uuid::new_v4(),
			address: "127.0.0.1".to_string(),

			output: reciever,
			input: sender,

			server_channel: Mutex::new(None),

			stream: Mutex::new(None),

			stream_reader: Mutex::new(None),
			stream_writer: Mutex::new(None),
		}
	}
}

// MARK: - used for sorting.
impl PartialEq for Client {
	fn eq(&self, other: &Self) -> bool {
		self.uuid == other.uuid
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
		self.uuid.cmp(&other.uuid)
	}
}
