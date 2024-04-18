use tokio::{
	sync::{
		mpsc::{unbounded_channel, UnboundedReceiver},
		Mutex,
	},
	task::JoinHandle,
};

use crate::{
	listener_manager::{ConnectionType, ListenerManager},
	network_connection::{NetworkConnection, ServerRequest},
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
					let conn = NetworkConnection::new(stream, addr);
					println!("[Server] New protobuf connection");
					self.handle_protobuf_connection(conn).await;
				}
			};
		}
	}

	async fn handle_protobuf_connection(&self, mut conn: NetworkConnection) {
		let req = conn.get_request().await.unwrap();

		match req {
			ServerRequest::GetInfo => {
				conn
					.send_info("test server".into(), "mickyb18a@gmail.com".into())
					.await
			}
			ServerRequest::Connect {
				username: _,
				uuid: _,
			} => todo!(),
			ServerRequest::Ignore => todo!(),
		}
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
