pub mod client_management;
pub mod network_manager;

use uuid::Uuid;
use crate::lib::server::network_manager::NetworkManager;
use std::sync::Arc;

use crossbeam_channel::{Receiver, unbounded};

use crate::lib::server::client_management::ClientManager;
use crate::lib::server::client_management::traits::TClientManager;
use crate::lib::foundation::{ICooperative};
use client_management::client::Client;

/// # ServerMessages
/// This is used internally 
#[derive(Debug)]
pub enum ServerMessages {
	ClientConnected(Arc<Client>),

  #[allow(dead_code)]
  ClientDisconnected(Uuid),
}

pub struct Server {
	client_manager: Arc<ClientManager>,
  network_manager: Arc<NetworkManager>,

	receiver: Receiver<ServerMessages>,
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
				ServerMessages::ClientConnected(client) => {
					self.client_manager.add_client(client);
				},
				ServerMessages::ClientDisconnected(uuid) => {
					self.client_manager.remove_client(uuid);
				}
			}
		}

		// alocate time for other components
		self.network_manager.tick();
		self.client_manager.tick();

	}
}