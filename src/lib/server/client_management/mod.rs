pub mod client;
mod traits;

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::Weak;

use crossbeam_channel::{unbounded, Receiver, Sender};

use uuid::Uuid;

use self::client::Client;
use self::client::ClientMessage;
// use client::client_v3::Client;
use self::traits::TClientManager;

enum ClientManagerMessages {}

/// # ClientManager
/// This struct manages all connected users
pub struct ClientManager {
  clients: Mutex<Vec<Arc<Client>>>,

  weak_self: Mutex<Option<Weak<Self>>>,

  sender: Sender<ClientManagerMessages>,
  receiver: Receiver<ClientManagerMessages>,
}

impl ClientManager {
  pub fn new() -> Arc<Self> {

    let channels = unbounded();

    let mut manager_ref: Arc<Self> = Arc::new(ClientManager {
      clients: Mutex::default(),

      weak_self: Mutex::default(),

      sender: channels.0,
      receiver: channels.1,
    });

    // get the reference
    {
      let mut lock = manager_ref.weak_self.lock().unwrap();
      let tmp = manager_ref.clone();
      *lock = Some(Arc::downgrade(&tmp));
    }

    manager_ref.set_ref(manager_ref.clone());
    manager_ref
  }

  pub fn get_ref(&self) -> Weak<Self> {
    self.weak_self.lock().unwrap().clone().unwrap()
  }

  fn set_ref(&self, reference: Arc<Self>) {
    let mut lock = self.weak_self.lock().unwrap();
    *lock = Some(Arc::downgrade(&reference));
  }
}

impl TClientManager<Client, ClientMessage> for ClientManager {
  fn addClient(&self, Client: std::sync::Arc<Client>) {
    self.clients.lock().unwrap().push(Client);
  }

  fn removeClient(&self, uuid: Uuid) {
    self.clients.lock().unwrap().sort();
  }

  fn messageClient(&self, id: Uuid, msg: ClientMessage) {
    todo!()
  }
  fn tick(&self) {
    todo!()
  }
}

#[cfg(test)]
mod test {
  use super::ClientManager;
  use std::sync::Arc;

  #[test]
  fn test_get_ref() {
    let mut clientManager = ClientManager::new();
    let cm_ref = clientManager.get_ref();
    assert_eq!(Arc::weak_count(&clientManager), 2);
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
