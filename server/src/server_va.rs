use std::net::SocketAddr;

use foundation::{
	networking::{read_message, write_message},
	prelude::{
		network_client_message,
		network_server_message,
		GetInfo,
		Info,
		NetworkClientMessage,
		NetworkServerMessage,
		Request,
	},
};
use tokio::{
	io::AsyncWriteExt,
	net::TcpStream,
	sync::{
		mpsc::{unbounded_channel, UnboundedReceiver},
		Mutex,
	},
	task::JoinHandle,
};

use crate::{
	listener_manager::{ConnectionType, ListenerManager},
	os_signal_manager::OSSignalManager,
};

pub struct Server {
	os_event_manager_task: JoinHandle<()>,
	listener_task: JoinHandle<()>,
	receiver: Mutex<UnboundedReceiver<ServerMessages>>,
}

impl Server {
	pub async fn run(&self) {
		loop {
			let mut lock = self.receiver.lock().await;
			let msg = lock.recv().await;
			drop(lock);

			match msg {
				Some(ServerMessages::Exit) | None => {
					println!("[Server] Shutting down");
					self.shutdown();
					return;
				}
				Some(ServerMessages::NewConnection(
					ConnectionType::ProtobufConnection(stream, addr),
				)) => {
					println!("[Server] New protobuf connection");
					self.handle_protobuf_connection(stream, addr).await;
				}
			};
		}
	}

	async fn handle_protobuf_connection(
		&self,
		mut stream: TcpStream,
		_addr: SocketAddr,
	) {
		let message = NetworkServerMessage {
			message: Some(network_server_message::Message::Request(Request {
				a: true,
			})),
		};

		println!("[Server] made message {:?}", message);
		write_message(&mut stream, message).await.unwrap();

		let request = read_message::<NetworkClientMessage>(&mut stream)
			.await
			.unwrap();

		match request {
			NetworkClientMessage {
				message: Some(network_client_message::Message::GetInfo(GetInfo {})),
			} => {
				let message = NetworkServerMessage {
					message: Some(network_server_message::Message::GotInfo(Info {
						server_name: "Test server".into(),
						owner: "mickyb18a@gmail.com".into(),
					})),
				};
				write_message(&mut stream, message).await.unwrap();
			}
			_ => {
				println!("[Server] message not supported");
			}
		}

		let _ = stream.flush().await;
	}

	fn shutdown(&self) {
		self.os_event_manager_task.abort();
		self.listener_task.abort();
	}
}

impl Default for Server {
	fn default() -> Self {
		let (tx, rx) = unbounded_channel();
		let tx1 = tx.clone();
		let tx2 = tx.clone();

		let os_event_manager_task = tokio::spawn(async move {
			OSSignalManager::new(tx1).run().await;
		});

		let listener_task = tokio::spawn(async move {
			ListenerManager::new(tx2).await.run().await;
		});

		Self {
			os_event_manager_task,
			receiver: Mutex::new(rx),
			listener_task,
		}
	}
}

pub enum ServerMessages {
	Exit,
	NewConnection(ConnectionType),
}
