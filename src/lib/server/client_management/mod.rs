pub mod client;
mod traits;

// use crate::lib::server::ServerMessages;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::Weak;

use crossbeam_channel::{unbounded, Receiver, Sender};
use uuid::Uuid;

use crate::lib::Foundation::{IOwner, IOwned};
use self::client::Client;
use self::client::ClientMessage;
use self::traits::TClientManager;
use crate::lib::Foundation::IMessagable;
use crate::lib::Foundation::ICooperative;

enum ClientManagerMessages {}

/// # ClientManager
/// This struct manages all connected users
pub struct ClientManager {
  clients: Mutex<Vec<Arc<Client>>>,

  weak_self: Mutex<Option<Weak<Self>>>,

	// server_channel: Sender<ServerMessages>,

  sender: Sender<ClientManagerMessages>,
  receiver: Receiver<ClientManagerMessages>,
}

impl ClientManager {
  pub fn new(/*server_channel:  Sender<ServerMessages> */) -> Arc<Self> {

    let (sender, receiver) = unbounded();

    Arc::new(ClientManager {
      clients: Mutex::default(),

      weak_self: Mutex::default(),

			// server_channel,

      sender,
      receiver,
    })
  }

  fn set_ref(&self, reference: Arc<Self>) {
    let mut lock = self.weak_self.lock().unwrap();
    *lock = Some(Arc::downgrade(&reference));
	}
}

impl TClientManager<Client, ClientMessage> for ClientManager {
  fn add_client(&self, client: std::sync::Arc<Client>) {
    self.clients.lock().unwrap().push(client);
  }

  fn remove_client(&self, _uuid: Uuid) {
    self.clients.lock().unwrap().sort();
  }

  fn message_client(&self, _id: Uuid, _msg: ClientMessage) {
    todo!()
  }

  fn tick(&self) {
		let client_list = self.clients.lock().unwrap();
		let _ = client_list.iter().map(|client| client.tick());
  }
}


#[cfg(test)]
mod test {
  use super::ClientManager;
  use std::sync::Arc;
	use crate::lib::Foundation::{IOwner};

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
