use tokio::{
	sync::{
		mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
		Mutex,
	},
	task::JoinHandle,
};

use crate::{
	connection::connection_manager::{
		ConnectionManager,
		ConnectionManagerMessage,
	},
	network::{
		listener_manager::{ConnectionType, ListenerManager},
		network_connection::{NetworkConnection, ServerRequest},
	},
	os_signal_manager::OSSignalManager,
};

/// # Server
/// Manages communication between components in the server
/// Main functions being the handling of new connections, and setting them up.
pub struct Server {
	connection_manager_sender: UnboundedSender<ConnectionManagerMessage>,
	connection_manager_task: JoinHandle<()>,
	listener_task: JoinHandle<()>,
	os_event_manager_task: JoinHandle<()>,
	receiver: Mutex<UnboundedReceiver<ServerMessages>>,
}

impl Server {
	/// Loops the future, reading messages from the servers channel.
	/// if exit is received, deconstructs all sub-tasks and exits the loop.
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
					let conn = NetworkConnection::new(stream, addr);
					println!("[Server] New protobuf connection");
					self.handle_protobuf_connection(conn).await;
				}
			};
		}
	}

	async fn handle_protobuf_connection(&self, mut conn: NetworkConnection) {
		println!("[Server] Getting request");
		let req = conn.get_request().await.unwrap();

		match req {
			ServerRequest::GetInfo => {
				conn
					.send_info("test server".into(), "mickyb18a@gmail.com".into())
					.await
			}
			ServerRequest::Connect {
				username,
				uuid,
				addr,
			} => {
				println!("[Server] sending connectionn and info to conneciton manager");
				self.connection_manager_sender.send(
					ConnectionManagerMessage::AddClient {
						conn,
						uuid,
						username,
						addr,
					},
				);
			}
			ServerRequest::Ignore => todo!(),
		}
	}

	fn shutdown(&self) {
		self.os_event_manager_task.abort();
		self.connection_manager_task.abort();
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

		let mut connection_manager = ConnectionManager::new();
		let connection_manager_sender = connection_manager.get_sender();
		let connection_manager_task = tokio::spawn(async move {
			connection_manager.run().await;
		});

		Self {
			os_event_manager_task,
			connection_manager_task,
			connection_manager_sender,
			receiver: Mutex::new(rx),
			listener_task,
		}
	}
}

/// # ServerMessage
/// enum describing all messages that the server can handle
pub enum ServerMessages {
	Exit,
	NewConnection(ConnectionType),
}
