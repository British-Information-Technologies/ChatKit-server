// use crate::lib::server::ServerMessages;
use crate::messages::ServerMessage;
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;

use crossbeam_channel::{unbounded, Receiver, Sender};
use uuid::Uuid;

use crate::client::Client;
use foundation::prelude::IMessagable;
use foundation::prelude::ICooperative;

enum ClientManagerMessage {
  #[allow(dead_code)]
  DropAll,
  #[allow(dead_code)]
  MessageClient,
}

/// # ClientManager
/// This struct manages all connected users
#[derive(Debug)]
pub struct ClientManager {
  clients: Mutex<HashMap<Uuid, Arc<Client>>>,

	server_channel: Sender<ServerMessage>,

  sender: Sender<ClientManagerMessage>,
  receiver: Receiver<ClientManagerMessage>,
}

impl ClientManager {
  pub fn new(server_channel:  Sender<ServerMessage>) -> Arc<Self> {

    let (sender, receiver) = unbounded();

    Arc::new(ClientManager {
      clients: Mutex::default(),

			server_channel,

      sender,
      receiver,
    })
  }
}

// impl TClientManager<Client, ClientMessage> for ClientManager {
//   fn add_client(&self, client: std::sync::Arc<Client>) {
//     self.clients.lock().unwrap().insert(client.uuid, client);
//   }

//   fn remove_client(&self, uuid: Uuid) {
//     let _ = self.clients.lock().unwrap().remove(&uuid);
//   }

//   fn send_message_to_client(&self, uuid: Uuid, msg: ClientMessage) {
//     let clients = self.clients.lock().unwrap();
//     let client = clients.get(&uuid).unwrap();
//     client.send_message(msg);
//   }
// }

impl IMessagable<ClientManagerMessage, Sender<ServerMessage>> for ClientManager {
	fn send_message(&self, msg: ClientManagerMessage) {

  }
  fn set_sender(&self, sender: Sender<ServerMessage>) {

  }
}

impl ICooperative for ClientManager {
  fn tick(&self) {

    for message in self.receiver.iter() {
      match message {
        ClientManagerMessage::DropAll => {
          println!("cannot drop all clients yet")
        }
        _ => println!("[Client Manager]: method not implemented")
      }
    }

    // allocate time for clients.
    let clients = self.clients.lock().unwrap();
    let _ = clients.iter().map(|(_uuid, client)| client.tick());
  }
}


#[cfg(test)]
mod test {
  // use super::ClientManager;
  // use std::sync::Arc;
	// use crate::lib::Foundation::{IOwner};

  #[test]
  fn test_get_ref() {
    // let client_manager = ClientManager::new();
    // let _cm_ref = client_manager.get_ref();
    // assert_eq!(Arc::weak_count(&client_manager), 2);
  }

  #[test]
  fn test_add_client() {
    todo!()
  }

  #[test]
  fn test_remove_client() {
    todo!()
  }

  #[test]
  fn test_remove_all_clients() {
    todo!()
  }
}
