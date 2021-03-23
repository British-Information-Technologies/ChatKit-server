use std::sync::Arc;
use std::thread;

use crossbeam_channel::{unbounded, Receiver};
use uuid::Uuid;

use crate::client_manager::ClientManager;
use crate::messages::ClientMgrMessage;
use crate::messages::ServerMessage;
use crate::network_manager::NetworkManager;
use foundation::prelude::ICooperative;
use foundation::prelude::IMessagable;
use foundation::prelude::IPreemptive;

/// # ServerMessages
/// This is used internally
#[derive(Debug)]
pub enum ServerMessages<TClient> {
	ClientConnected(Arc<TClient>),

	#[allow(dead_code)]
	ClientDisconnected(Uuid),
}

pub struct Server {
	client_manager: Arc<ClientManager>,
	network_manager: Arc<NetworkManager>,

	receiver: Receiver<ServerMessage>,
}

impl Server {
	pub fn new() -> Arc<Server> {
		let (sender, receiver) = unbounded();

		Arc::new(Server {
			client_manager: ClientManager::new(sender.clone()),

			network_manager: NetworkManager::new("5600".to_string(), sender),
			receiver,
		})
	}
}

impl ICooperative for Server {
	fn tick(&self) {
		println!("[server]: Tick!");
		use ClientMgrMessage::{Add, Remove};

		// handle new messages loop
		if !self.receiver.is_empty() {
			println!("[server]: entering loop!");
			for message in self.receiver.try_iter() {
				println!("[server]: received message {:?}", &message);
				match message {
					ServerMessage::ClientConnected(client) => {
						self.client_manager.send_message(Add(client))
					}
					ServerMessage::ClientDisconnected(uuid) => {
						println!("disconnecting client {:?}", uuid);
						self.client_manager.send_message(Remove(uuid));
					}
				}
			}
		}
	}
}

impl IPreemptive for Server {
	fn run(arc: &std::sync::Arc<Self>) {
		// start services
		NetworkManager::start(&arc.network_manager);
		ClientManager::start(&arc.client_manager);
		loop {
			thread::sleep(std::time::Duration::from_secs(1));
			arc.tick();
		}
	}

	fn start(arc: &std::sync::Arc<Self>) {
		let arc = arc.clone();
		// start thread
		std::thread::spawn(move || Server::run(&arc));
	}
}
