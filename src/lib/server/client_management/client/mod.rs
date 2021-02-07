// pub mod client_profile;
// pub mod client_v3;
pub mod traits;

use serde::{Serialize, Deserialize};
use std::net::TcpStream;
use std::sync::Weak;
use uuid::Uuid;

use super::traits::TClientManager;
use super::ClientManager;

pub enum ClientMessage {
  a,
  b,
}

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

