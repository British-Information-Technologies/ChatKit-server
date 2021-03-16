// pub mod client_profile;
// pub mod client_v3;
pub mod traits;

use std::sync::Mutex;
use std::net::TcpStream;
use std::sync::Weak;
use std::sync::Arc;
use std::cmp::Ordering;
use std::mem;

use uuid::Uuid;
use serde::Serialize;
use crossbeam_channel::{Sender, Receiver, unbounded};

use crate::lib::Foundation::{IOwned, ICooperative, IMessagable};
use super::ClientManager;
use traits::IClient;

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
#[derive(Debug, Serialize)]
pub struct Client {
  pub uuid: Uuid,
  username: String,
  address: String,

	// non serializable
	#[serde(skip)]
	output_channel: Mutex<Receiver<ClientMessage>>,

	#[serde(skip)]
	input_channel: Mutex<Sender<ClientMessage>>,

	#[serde(skip)]
  stream: Mutex<Option<TcpStream>>,

	#[serde(skip)]
  owner: Mutex<Option<Weak<ClientManager>>>

}

// client funciton implmentations
impl IClient<ClientMessage> for Client {
  fn new(uuid: Uuid, name: String, addr: String) -> Arc<Client> {
		let (sender, reciever) = unbounded();

    Arc::new(Client {
      username: name,
      uuid: Uuid::new_v4(),
      address: addr,

			output_channel: Mutex::new(reciever),
			input_channel: Mutex::new(sender),

      stream: Mutex::new(None),
      owner: Mutex::new(None)
    })
  }

	// MARK: - removeable
  fn send(&self, bytes: Vec<u8>) -> Result<(), &str> { todo!() }
  fn recv(&self) -> Option<Vec<u8>> { todo!() }
	// Mark: end -
}

impl IOwned<ClientManager> for Client {
  fn set_owner(&self, owner: Weak<ClientManager>) {
    let mut owner_mut = self.owner.lock().unwrap();
    let _ = mem::replace(&mut *owner_mut, Some(owner));
  }
}

impl IMessagable<ClientMessage> for Client{
	fn send_message(&self, msg: ClientMessage) {
		self.input_channel.lock().unwrap().send(msg);
	}
}

// cooperative multitasking implementation
impl ICooperative for Client {
	fn tick(&self) {
	}
}

// default value implementation
impl Default for Client {
	fn default() -> Self {
		let (sender, reciever) = unbounded();
		return Client {
			username: "generic_client".to_string(),
      uuid: Uuid::new_v4(),
      address: "127.0.0.1".to_string(),

			output_channel: Mutex::new(reciever),
			input_channel: Mutex::new(sender),

      stream: Mutex::new(None),
      owner: Mutex::new(None)
		}
	}
}

// MARK: - used for sorting.
impl PartialEq for Client {
      fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl Eq for Client {
}

impl PartialOrd for Client {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
			Some(self.cmp(other))
	}
}

impl Ord for Client {
      fn cmp(&self, other: &Self) -> Ordering {
        self.uuid.cmp(&other.uuid)
    }
}
