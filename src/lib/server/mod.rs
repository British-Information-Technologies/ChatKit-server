pub mod client_management;

use crossbeam_channel::{Sender, Receiver, unbounded};
use std::sync::{Arc, Weak, Mutex};
use std::net::TcpListener;

use crate::lib::server::client_management::ClientManager;
use crate::lib::Foundation::{IOwner, IOwned, ICooperative};
use client_management::client::Client;

enum ServerMessages {
	ClientConnected(Client),
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
			client_manager: ClientManager::new(sender),
			
			sender,
			receiver,
		})
	}
}

impl ICooperative for Server{
	fn tick(&self) {

	}
}
