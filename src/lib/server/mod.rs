pub mod client_management;


use std::sync::{Arc, Weak, Mutex};
use std::collections::HashMap;
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
      }
    }
	}
}
