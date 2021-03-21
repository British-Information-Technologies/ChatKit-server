use uuid::Uuid;
use std::sync::Arc;

use crate::client::Client;

pub enum ClientMessage {
  Message(Uuid, String),

  Disconnect,
}

pub enum ClientMgrMessage {
  Remove(Uuid),
  Add(Arc<Client>),
  SendMessage(Uuid, Uuid, String),
}

pub enum ServerMessage {
  ClientConnected(Arc<Client>),
  ClientDisconnected(Uuid)
}