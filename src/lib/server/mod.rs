pub mod client_management;

use std::sync::{Arc, Weak, Mutex};
use std::net::TcpListener;

use crate::lib::server::client_management::ClientManager;
use crate::lib::Foundation::{IOwner, IOwned, ICooperative};

pub struct Server {
	server_socket: TcpListener,

	client_manager: Arc<ClientManager>,
}

impl Server {
	pub fn new() -> Arc<Server> {

		let listener = TcpListener::bind("0.0.0.0:5600").expect("Could not bind to address");

		let server: Arc<Self> = Arc::new(Server {
			server_socket: listener,
			weak_self: Mutex::new(None),
			client_manager: ClientManager::new()
		});

		server.
	}


}

impl IOwner<ClientManager> for Server {
	fn add_child(&self, child: Arc<ClientManager>) {
		self.client_manager
	}

	fn get_ref(&self) -> Weak<Self> {
		self.weak_self.lock().unwrap().unwrap().clone()
	}
}

impl ICooperative for Server{
	fn tick(&self) {

	}
}
