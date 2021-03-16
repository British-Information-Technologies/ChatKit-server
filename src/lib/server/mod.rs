pub mod client_management;


use std::sync::{Arc, Weak, Mutex};
use std::net::TcpListener;
use std::io::Write;
use std::io::Read;

use crossbeam_channel::{Sender, Receiver, unbounded};

use crate::lib::server::client_management::ClientManager;
use crate::lib::Foundation::{IOwner, IOwned, ICooperative};
use client_management::client::Client;
use crate::lib::commands::Commands;

#[derive(Debug)]
pub enum ServerMessages {
	ClientConnected(Arc<Client>),
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
}

impl ICooperative for Server{
	fn tick(&self) {

    let mut buffer = vec![0; 1024];

    // get connections 
    for connection in self.server_socket.incoming() {
      match connection {
        Ok(mut stream) => {
          let _ = stream.write(Commands::Request(None).to_string().as_bytes());
          let _ = stream.read(&mut buffer);

          let command = Commands::from(&mut buffer);

          match command {
            Commands::Info(None) => {let _ = stream.write("todo".as_bytes());}
            _ => {let _ = stream.write("not implemented!".as_bytes());}
          }

        },
        _ => println!("!connection error occured!"),
      }
    }



    // message loop
    for message in self.receiver.iter() {
      match message {
        ServerMessages::ClientConnected(client) => println!("client connected: {:?}", client),
      }
    }
	}
}
