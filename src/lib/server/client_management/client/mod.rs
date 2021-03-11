// pub mod client_profile;
// pub mod client_v3;
pub mod traits;

use serde::{Serialize, Deserialize};
use std::net::TcpStream;
use std::sync::Weak;
use std::sync::Arc;
use uuid::Uuid;
use std::cmp::Ordering;

use super::ClientManager;
use traits::TClient;

pub enum ClientMessage {
  a,
  b,
}

/// # Client
/// This struct represents a connected user.
/// 
/// ## Attrubutes
/// - uuid: The id of the connected user.
/// - username: The username of the connected user.
/// - address: The the address of the connected client.
/// 
/// - stream: The socket for the connected client.
/// - owner: An optional reference to the owning object.
#[derive(Serialize, Deserialize)]
pub struct Client {
  uuid: String,
  username: String,
  address: String,

  #[serde(skip)]
  stream: Option<TcpStream>,

  #[serde(skip)]
  owner: Option<Weak<ClientManager>>
}

impl TClient<ClientMessage> for Client {
  fn new(uuid: Uuid, name: String, addr: String) -> Arc<Client> { todo!() }

  fn send(&self, bytes: Vec<u8>) -> Result<(), &str> { todo!() }
  fn recv(&self) -> Option<Vec<u8>> { todo!() }

  fn send_msg(&self, msg: ClientMessage) -> Result<(), &str> { todo!() }
  fn recv_msg(&self) -> Option<ClientMessage> { todo!() }

  fn tick(&self) {  }
}

impl PartialEq for Client {
      fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl Eq for Client {
}

impl Ord for Client {
      fn cmp(&self, other: &Self) -> Ordering {
        self.uuid.cmp(&other.uuid)
    }
}

impl PartialOrd for Client {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}