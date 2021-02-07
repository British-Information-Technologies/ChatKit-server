mod traits;
pub mod client;

use std::sync::Weak;
use std::sync::Arc;

use crossbeam_channel::{Sender, Receiver};

use uuid::Uuid;

use self::client::Client;
use self::client::ClientMessage;
// use client::client_v3::Client;
use self::traits::TClientManager;



enum ClientManagerMessages {

}

pub struct ClientManager {
  clients: Vec<Arc<Client>>,

  weak_self: Option<Weak<Self>>,

  sender: Sender<ClientManagerMessages>,
  receiver: Receiver<ClientManagerMessages>,
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