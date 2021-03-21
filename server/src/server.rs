use std::sync::Arc;

use uuid::Uuid;
use crossbeam_channel::{Receiver, unbounded};

use foundation::prelude::ICooperative;
use foundation::prelude::IMessagable;
use crate::client_manager::ClientManager;
use crate::network_manager::NetworkManager;
use crate::messages::ClientMgrMessage;
use crate::messages::ServerMessage;

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

impl ICooperative for Server{
	fn tick(&self) {
    println!("[server]: Tick!");
    use ClientMgrMessage::{Remove, Add};



		// handle new messages loop
    
    if !self.receiver.is_empty() {
      println!("[server]: entering loop!");
      for message in self.receiver.try_iter() {
        println!("[server]: received message {:?}", &message);
        match message {
          ServerMessage::ClientConnected(client) => {
            self.client_manager.send_message(Add(client))
          },
          ServerMessage::ClientDisconnected(uuid) => {
            println!("disconnecting client {:?}", uuid);
            self.client_manager.send_message(Remove(uuid));
          }
        }
      }
    }

		// alocate time for other components
    println!("[server]: allocating time for others");
		self.network_manager.tick();
		self.client_manager.tick();
	}
}
