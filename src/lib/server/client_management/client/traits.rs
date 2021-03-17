use std::sync::Arc;
use std::collections::HashMap;

use crossbeam_channel::Sender;

use crate::lib::server::ServerMessages;

/// # TClient
/// This trait represents the methods that a client must implement
/// in order to be used with a client manager
/// 
/// # Methods
/// - new: creates a new client from an id, username and a address.
/// - send: send a message to the client.
/// - recv: if there is a message in the queue, returns the message
/// - send_msg: sends a event message to the client
/// - recv_msg: used by the client to receive and process event messages
pub trait IClient<TClientMessage> {
  fn new(map: HashMap<String, String>, server_channel: Sender<ServerMessages> ) -> Arc<Self>;

  fn send(&self, bytes: Vec<u8>) -> Result<(), &str>;
  fn recv(&self) -> Option<Vec<u8>>;
}