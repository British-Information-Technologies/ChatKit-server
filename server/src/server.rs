use crate::messages::ServerMessage;
use uuid::Uuid;

use std::sync::Arc;
use crossbeam_channel::{Receiver, unbounded};

use foundation::prelude::ICooperative;
use crate::client_manager::ClientManager;
use crate::network_manager::NetworkManager;

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

      network_manager: NetworkManager::new("5600".to_string(), sender.clone()),
			receiver,
		})
	}
}

impl ICooperative for Server{
	fn tick(&self) {

		// handle new messages loop
		for message in self.receiver.try_iter() {
			match message {
				ServerMessage::ClientConnected(client) => {
				},
				ServerMessage::ClientDisconnected(uuid) => {
				}
			}
		}

		// alocate time for other components
		self.network_manager.tick();
		self.client_manager.tick();

	}
}
