
use std::sync::Arc;
use crate::lib::server::ServerMessages;
use std::net::TcpListener;

use serde::{Deserialize, Serialize};
use crossbeam_channel::Sender;

use crate::lib::Foundation::ICooperative;

#[derive(Serialize, Deserialize)]
enum NetworkSocketMesssages {
  Info {id: String},
  Connect {id: String, uuid: String, username: String, address: String},
}

pub enum NetworkMessages {

}

pub struct NetworkManager {
  listener: TcpListener,

  server_channel: Sender<ServerMessages>,
}

impl NetworkManager {
  pub fn new(port: String, server_channel: Sender<ServerMessages>) -> Arc<NetworkManager> {
    let mut address = "0.0.0.0:".to_string();
    address.push_str(&port);

		let listener = TcpListener::bind(address)
      .expect("Could not bind to address");

    Arc::new(NetworkManager {
      listener,
      server_channel,
    })
  }
}

impl ICooperative for NetworkManager {
  fn tick(&self) {
    println!("network manager tick")
  }
}