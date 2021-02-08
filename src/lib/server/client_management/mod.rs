mod traits;
pub mod client;

use std::sync::Weak;
use std::sync::Arc;
use std::sync::Mutex;

use crossbeam_channel::{Sender, Receiver, unbounded};

use uuid::Uuid;

use self::client::Client;
use self::client::ClientMessage;
// use client::client_v3::Client;
use self::traits::TClientManager;

enum ClientManagerMessages {

}

/// # ClientManager
/// This struct manages all connected users
pub struct ClientManager {
  clients: Vec<Arc<Client>>,

  weak_self: Mutex<Option<Weak<Self>>>,

  sender: Sender<ClientManagerMessages>,
  receiver: Receiver<ClientManagerMessages>,
}

impl ClientManager {
  pub fn new() -> Arc<Self> {
    let channels = unbounded();


    let mut manager_ref: Arc<Self> = Arc::new(ClientManager {
      clients: Vec::default(),

      weak_self: Mutex::default(),

      sender: channels.0,
      receiver: channels.1,
    });

    manager_ref.set_ref(manager_ref.clone());

    manager_ref
  }

  pub fn get_ref(&self) -> Arc<Self>{
    let new_ref: Weak<Self> = self.weak_self.lock().unwrap().clone().unwrap();
    new_ref.upgrade().unwrap()
  }

  fn set_ref(&self, reference: Arc<Self>) {
    let mut lock = self.weak_self.lock().unwrap();
    *lock = Some(Arc::downgrade(&reference));
  }
}

impl TClientManager<Client, ClientMessage> for ClientManager {
  fn addClient(&self, Client: std::sync::Arc<Client>) { todo!() }

  fn removeClient(&self, uuid: Uuid) { todo!() }

  fn messageClient(&self, id: Uuid, msg: ClientMessage) { todo!() }
  fn tick(&self) { todo!() }
}


#[cfg(test)]
mod test {

    #[test]
    fn test_add_client() { todo!() }

    #[test]
    fn test_remove_client() { todo!() }

    #[test]
    fn test_remove_all_clients() { todo!() }
}