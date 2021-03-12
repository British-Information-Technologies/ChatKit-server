pub mod client;
mod traits;

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

  sender: Sender<ClientManagerMessages>,
  receiver: Receiver<ClientManagerMessages>,
}

impl ClientManager {
  pub fn new() -> Arc<Self> {

    let channels = unbounded();

    let manager_ref: Arc<Self> = Arc::new(ClientManager {
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



  fn set_ref(&self, reference: Arc<Self>) {
    let mut lock = self.weak_self.lock().unwrap();
    *lock = Some(Arc::downgrade(&reference));
  }
}

impl TClientManager<Client, ClientMessage> for ClientManager {
  fn add_client(&self, client: std::sync::Arc<Client>) {
    self.add_child(client);
  }

  fn remove_client(&self, uuid: Uuid) {
    let mut client_list = self.clients.lock().unwrap();
		client_list.sort();
		if let Ok(index) = client_list.binary_search_by(move |client| client.uuid.cmp(&uuid)) {
			client_list.remove(index);
		}
  }

  fn message_client(&self, id: Uuid, msg: ClientMessage) -> Result<(), &str> {
    let mut client_list = self.clients.lock().unwrap();
		client_list.sort();
		if let Ok(index) = client_list.binary_search_by(move |client| client.uuid.cmp(&id)) {
			if let Some(client) = client_list.get(index) {
				let _ = client.send_message(msg);
			} 
		}
		Ok(())
  }

  fn tick(&self) {
		let client_list = self.clients.lock().unwrap();
		let _ = client_list.iter().map(|client| client.tick());
  }
}

impl IOwner<Client> for ClientManager{
  fn add_child(&self, child: Arc<Client>) {
		child.set_owner(self.get_ref());
    self.clients.lock().unwrap().push(child);
  }

	fn get_ref(&self) -> Weak<Self> {
    self.weak_self.lock().unwrap().clone().unwrap()
  }
}

#[cfg(test)]
mod test {
  use super::ClientManager;
  use std::sync::Arc;
	use crate::lib::Foundation::{IOwner};

  #[test]
  fn test_get_ref() {
    let client_manager = ClientManager::new();
    let _cm_ref = client_manager.get_ref();
    assert_eq!(Arc::weak_count(&client_manager), 2);
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
