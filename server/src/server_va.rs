use foundation::prelude::GlobalMessage;
use tokio::{
	sync::{
		mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
		Mutex,
	},
	task::JoinHandle,
};
use uuid::Uuid;

use crate::{
	chat::ChatManager,
	connection::connection_manager::{
		ConnectionManager,
		ConnectionManagerMessage,
	},
	network::{
		json::{
			json_listener::JSONListener,
			json_network_connection::JSONNetworkConnection,
		},
		protobuf::{
			protobuf_listener::ProtobufListener,
			protobuf_network_connection::ProtobufNetworkConnection,
		},
		ConnectionType,
		NetworkConnection,
		NetworkListener,
		ServerRequest,
	},
	os_signal_manager::OSSignalManager,
};

/// # Server
/// Manages communication between components in the server
/// Main functions being the handling of new connections, and setting them up.
pub struct Server {
	connection_manager_sender: UnboundedSender<ConnectionManagerMessage>,

	chat_manager: ChatManager,

	connection_manager_task: JoinHandle<()>,
	listener_task: JoinHandle<()>,
	json_listener_task: JoinHandle<()>,

	os_event_manager_task: JoinHandle<()>,

	receiver: Mutex<UnboundedReceiver<ServerMessages>>,
}

impl Server {
	/// Loops the future, reading messages from the servers channel.
	/// if exit is received, deconstructs all sub-tasks and exits the loop.
	pub async fn run(&mut self) {
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
					let conn = Box::new(ProtobufNetworkConnection::new(stream, addr));
					println!("[Server] New protobuf connection");
					self.handle_protobuf_connection(conn).await;
				}
				Some(ServerMessages::NewConnection(
					ConnectionType::JsonConnection(stream, addr),
				)) => {
					let conn = Box::new(JSONNetworkConnection::new(stream, addr));
					println!("[Server] New protobuf connection");
					self.handle_protobuf_connection(conn).await;
				}
				Some(ServerMessages::SendGlobalMessages(uuid)) => {
					let messages = self.chat_manager.get_messages();
					println!("[Server] Sending Global Messages");
					_ = self.connection_manager_sender.send(
						ConnectionManagerMessage::SendGlobalMessagesTo { uuid, messages },
					);
				}
				Some(ServerMessages::AddGlobalMessage(message)) => {
					self.chat_manager.add_message(message);
				}
			};
		}
	}

	async fn handle_protobuf_connection(
		&self,
		mut conn: Box<dyn NetworkConnection>,
	) {
		println!("[Server] Getting request");
		let req = conn.get_request().await;

		let Ok(req) = req else {
			println!("[Server] Got invalid request");
			return;
		};

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
				_ = self.connection_manager_sender.send(
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
		self.json_listener_task.abort();
		self.listener_task.abort();
	}
}

impl Default for Server {
	fn default() -> Self {
		let (tx, rx) = unbounded_channel();
		let tx1 = tx.clone();
		let tx2 = tx.clone();
		let tx3 = tx.clone();
		let tx4 = tx.clone();

		let os_event_manager_task = tokio::spawn(async move {
			OSSignalManager::new(tx1).run().await;
		});

		let listener_task = ProtobufListener::start_run(tx2);
		let json_listener_task = JSONListener::start_run(tx3);

		let mut connection_manager = ConnectionManager::new(tx4);
		let connection_manager_sender = connection_manager.get_sender();
		let connection_manager_task = tokio::spawn(async move {
			connection_manager.run().await;
		});

		let chat_manager = ChatManager::new();

		Self {
			chat_manager,

			os_event_manager_task,
			connection_manager_task,
			connection_manager_sender,

			json_listener_task,
			receiver: Mutex::new(rx),
			listener_task,
		}
	}
}

/// # ServerMessage
/// enum describing all messages that the server can handle
pub enum ServerMessages {
	Exit,
	AddGlobalMessage(GlobalMessage),
	SendGlobalMessages(Uuid),
	NewConnection(ConnectionType),
}
