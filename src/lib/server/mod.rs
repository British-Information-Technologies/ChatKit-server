pub mod client_management;
pub mod network_manager;

use crate::lib::server::network_manager::NetworkManager;
use std::collections::HashMap;
use std::net::TcpListener;
use std::sync::Arc;
use std::io::Write;
use std::io::Read;


use uuid::Uuid;
use crossbeam_channel::{Sender, Receiver, unbounded};

use crate::lib::server::client_management::ClientManager;
use crate::lib::server::client_management::traits::TClientManager;
use crate::lib::Foundation::{ICooperative};
use client_management::client::Client;
use crate::lib::commands::Commands;

/// # ServerMessages
/// This is used internally 
#[derive(Debug)]
pub enum ServerMessages {
	ClientConnected(Arc<Client>),
  ClientDisconnected(Uuid)
}

pub struct Server {
	client_manager: Arc<ClientManager>,
  network_manager: Arc<NetworkManager>,

	sender: Sender<ServerMessages>,
	receiver: Receiver<ServerMessages>,
}

impl Server {
	pub fn new() -> Arc<Server> {
		let (sender, receiver) = unbounded();

		Arc::new(Server {
			client_manager: ClientManager::new(sender.clone()),

      network_manager: NetworkManager::new("5600".to_string(), sender.clone()),
			
			sender,
			receiver,
		})
	}

  pub fn send_message(&self, msg: ServerMessages) {
    self.sender.send(msg).expect("!error sending message to server!")
  }
}

impl ICooperative for Server{
	fn tick(&self) {

		self.network_manager.tick();

    // handle new messages loop
    for message in self.receiver.iter() {
      match message {
        ServerMessages::ClientConnected(client) => println!("client connected: {:?}", client),
        ServerMessages::ClientDisconnected(uuid) => {self.client_manager.remove_client(uuid);}
      }
    }
	}
}
