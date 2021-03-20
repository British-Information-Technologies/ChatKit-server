
use serde::{Serialize, Deserialize};

/// # ClientMessage
/// This enum defined the message that a client can receive from the server
/// This uses the serde library to transform to and from json. 
#[derive(Serialize, Deserialize)]
pub enum ClientStreamIn {
  Disconnect {id: String},
}

pub enum ClientStreamOut {
  Message {from_uuid: String},
  Disconnect,
}