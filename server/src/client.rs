use crate::messages::ClientMessage;
use crate::messages::ClientMessage::Disconnect;
use crate::messages::ServerMessage;
use foundation::prelude::IPreemptive;
use std::cmp::Ordering;
use std::io::BufRead;
use std::io::Read;
use std::io::Write;
use std::io::{BufReader, BufWriter};
use std::mem::replace;
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::Mutex;

use crossbeam_channel::{unbounded, Receiver, Sender};
use serde::Serialize;
use uuid::Uuid;

use foundation::messages::client::{ClientStreamIn, ClientStreamOut};
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
impl IPreemptive for Client {
	fn run(arc: &Arc<Self>) {
		let arc1 = arc.clone();
		let arc2 = arc.clone();

		// read thread
		std::thread::spawn(move || {
			let arc = arc1.clone();
			let mut buffer = String::new();
			let mut reader_lock = arc.stream_reader.lock().unwrap();
			let reader = reader_lock.as_mut().unwrap();

			while let Ok(size) = reader.read_line(&mut buffer) {
				if size == 0 {
					arc.send_message(Disconnect);
					break;
				}

				let command =
					serde_json::from_str::<ClientStreamIn>(buffer.as_str())
						.unwrap();

				match command {
					ClientStreamIn::Disconnect => arc.send_message(Disconnect),
					_ => println!("[client]: command not found"),
				}
			}
		});

		// write thread
		std::thread::spawn(move || {
			let arc = arc2.clone();
			let mut writer_lock = arc.stream_writer.lock().unwrap();
			let writer = writer_lock.as_mut().unwrap();

			let mut buffer: Vec<u8> = Vec::new();

			writeln!(
				buffer,
				"{}",
				serde_json::to_string(&ClientStreamOut::Connected).unwrap()
			);
			writer.write_all(&buffer).unwrap();
			writer.flush().unwrap();

			loop {
				for message in arc.output.iter() {
					match message {
						Disconnect => {
							arc.server_channel
								.lock()
								.unwrap()
								.as_mut()
								.unwrap()
								.send(ServerMessage::ClientDisconnected(
									arc.uuid,
								))
								.unwrap();
							break;
						}
						_ => println!("[client]: message not implemented"),
					}
				}
			}
		});
	}

	fn start(arc: &Arc<Self>) {
		Client::run(arc)
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

impl Drop for Client {
	fn drop(&mut self) {
		println!("[Client] dropped!");
	}
}
