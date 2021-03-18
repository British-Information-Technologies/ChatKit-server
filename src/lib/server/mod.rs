pub mod client_management;
pub mod network_manager;

use crate::lib::server::network_manager::NetworkManager;
use std::collections::HashMap;
use std::net::TcpListener;
use std::sync::Arc;
use std::io::Write;
use std::io::Read;


use uuid::Uuid;
use crossbeam_channel::{Sender, Receiver, unbounded};

use crate::lib::server::client_management::ClientManager;
use crate::lib::server::client_management::traits::TClientManager;
use crate::lib::Foundation::{ICooperative};
use client_management::client::Client;
use crate::lib::commands::Commands;

/// # ServerMessages
/// This is used internally 
#[derive(Debug)]
pub enum ServerMessages {
	ClientConnected(Arc<Client>),
  ClientDisconnected(Uuid)
}

pub struct Server {
	server_socket: TcpListener,
	client_manager: Arc<ClientManager>,
  network_manager: Arc<NetworkManager>,

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

      network_manager: NetworkManager::new("5600".to_string(), sender.clone()),
			
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

    let mut buffer = vec![0; 64];

    // handle new connections 
    for connection in self.server_socket.incoming() {
      match connection {
        Ok(mut stream) => {
          stream.write_all(Commands::Request(None).to_string().as_bytes()).expect("error writing socket");
          stream.read_to_end(&mut buffer).expect("error reading sokcet");
          
          println!("buffer: {:?}", &buffer);

          let command = Commands::from(&mut buffer);

          match command {
            Commands::Info(None) => {
              let server_config = vec![
                ("name".to_string(), "Test server".to_string())
              ];
              let map: HashMap<String, String> = server_config.into_iter().collect();
              stream.write_all(Commands::Success(Some(map)).to_string().as_bytes())
                .expect("error sending response");
            }
            Commands::Connect(Some(map)) => println!("connect command: {:?}", &map),

            _ => {let _ = stream.write("not implemented!".as_bytes());}
          }
        },
        _ => println!("!connection error occured!"),
      }
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
