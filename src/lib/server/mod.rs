pub mod client_management;


use std::io::BufWriter;
use std::io::BufReader;
use std::net::TcpListener;
use std::sync::Arc;
use std::io::Write;
use std::io::BufRead;

use uuid::Uuid;
use crossbeam_channel::{Sender, Receiver, unbounded};
use serde::{Serialize, Deserialize};

use crate::lib::server::client_management::ClientManager;
use crate::lib::server::client_management::traits::TClientManager;
use crate::lib::Foundation::{ICooperative};
use client_management::client::Client;

/// # ServerMessages
/// This is used internally 
#[derive(Debug)]
pub enum ServerMessages {
	ClientConnected(Arc<Client>),
  ClientDisconnected(Uuid)
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerSocketMessages {
	Request,
	Info,
	Connect {uuid: Uuid, username: String, address: String}
}

pub struct Server {
	server_socket: TcpListener,
	client_manager: Arc<ClientManager>,

	sender: Sender<ServerMessages>,
	receiver: Receiver<ServerMessages>,
}

impl Server {
	pub fn new() -> Arc<Server> {
		let listener = TcpListener::bind("0.0.0.0:5600").expect("Could not bind to address");
		let (sender, receiver) = unbounded();

		Arc::new(Server {
			server_socket: listener,
			client_manager: ClientManager::new(sender.clone()),
			
			sender,
			receiver,
		})
	}

  pub fn send_message(&self, msg: ServerMessages) {
    self.sender.send(msg).expect("!error sending message to server!")
  }
}

impl ICooperative for Server{
	fn tick(&self) {

    let mut buffer = String::new();

    // handle new connections 
    for connection in self.server_socket.incoming() {
      let (mut reader, mut writer) = match connection {
        Ok(mut stream) => (BufReader::new(stream.try_clone().unwrap()), BufWriter::new(stream.try_clone().unwrap())),
        Err(_) => break,
      };

			writer.write_all(serde_json::to_string(&ServerSocketMessages::Request).unwrap().as_bytes());
			writer.flush();

			reader.read_line(&mut buffer);

			println!("recieved: {:?}", &buffer);

			let msg: ServerSocketMessages = serde_json::from_str(&buffer).unwrap();

			println!("got msg: {:?}", msg)
    }

    // handle new messages loop
    for message in self.receiver.iter() {
      match message {
        ServerMessages::ClientConnected(client) => println!("client connected: {:?}", client),
        ServerMessages::ClientDisconnected(uuid) => {self.client_manager.remove_client(uuid);}
      }
    }
	}
}
