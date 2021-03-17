// pub mod client_profile;
// pub mod client_v3;
pub mod traits;

use std::collections::HashMap;
use std::cmp::Ordering;
use std::net::TcpStream;
use std::sync::Mutex;
use std::sync::Arc;

use uuid::Uuid;
use serde::Serialize;
use crossbeam_channel::{Sender, Receiver, unbounded};

use traits::IClient;
use crate::lib::Foundation::{ICooperative, IMessagable};
use crate::lib::server::ServerMessages;

pub enum ClientMessage {}

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
  server_channel: Option<Sender<ServerMessages>>,

  #[serde(skip)]
  input: Sender<ClientMessage>,

  #[serde(skip)]
  output: Receiver<ClientMessage>,

	#[serde(skip)]
  stream: Mutex<Option<TcpStream>>,
}

// client funciton implmentations
impl IClient<ClientMessage> for Client {
  fn new(map: HashMap<String, String>, server_channel: Sender<ServerMessages> ) -> Arc<Client> {
    let (sender, receiver) = unbounded();

    Arc::new(Client {
      username: map.get(&"name".to_string()).unwrap().clone(),
      uuid: Uuid::parse_str(map.get(&"uuid".to_string()).unwrap().as_str()).expect("invalid id"),
      address: map.get(&"host".to_string()).unwrap().clone(),

      server_channel: Some(server_channel),

      input: sender,
      output: receiver,

      stream: Mutex::new(None),
    })
  }

	// MARK: - removeable
  fn send(&self, _bytes: Vec<u8>) -> Result<(), &str> { todo!() }
  fn recv(&self) -> Option<Vec<u8>> { todo!() }
	// Mark: end -
}

impl IMessagable<ClientMessage> for Client{
	fn send_message(&self, msg: ClientMessage) {
		self.input.send(msg).expect("failed to send message to client.");
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
		Client {
			username: "generic_client".to_string(),
      uuid: Uuid::new_v4(),
      address: "127.0.0.1".to_string(),

		  output: reciever,
			input: sender,

      server_channel: None,

      stream: Mutex::new(None),
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
