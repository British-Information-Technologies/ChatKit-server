use std::io::{Error, ErrorKind};
use std::sync::Arc;
use std::time::Duration;
use cursive::views::{Dialog, TextView};
use tokio::sync::Mutex;
use tokio::time::sleep;
use async_trait::async_trait;
use tokio::net::ToSocketAddrs;
use tokio::sync::mpsc::Sender;

use serverlib::Server;

use foundation::ClientDetails;
use foundation::connection::Connection;
use foundation::messages::client::{ClientStreamIn, ClientStreamOut};
use foundation::messages::network::{NetworkSockIn, NetworkSockOut};
use foundation::prelude::IManager;
use crate::managers::NetworkManagerMessage;

pub struct NetworkManager<M>
	where M: From<NetworkManagerMessage> {
	server_connection: Mutex<Option<Connection>>,
	cursive: Sender<M>,
}

impl<M> NetworkManager<M>
	where M: From<NetworkManagerMessage> {
	pub fn new(sender: Sender<M>) -> Arc<Self> {
		Arc::new(NetworkManager {
			server_connection: Mutex::new(None),
			cursive: sender,
		})
	}
	
	pub async fn info<T: ToSocketAddrs>(self: &Arc<Self>, host: T) -> Result<NetworkManagerMessage, Error> {
		let connection= Connection::new();
		println!("Created connection");
		connection.connect(host).await?;
		let req = connection.read().await?;

		println!("request: {:?}", req);

		if let NetworkSockOut::Request =  req {
			connection.write::<NetworkSockIn>(NetworkSockIn::Info)
				.await?;
			return Ok(connection.read::<NetworkSockOut>()
				.await?.into());
		} else {
			Err(Error::new(ErrorKind::ConnectionAborted, "Request not received"))
		}
	}
	
	pub async fn login(self: &Arc<Self>, host: String, id: String, username: String) {
		let connection= Connection::new();
		let _ = connection.connect(host);
		
		// connection.write(NetworkSockIn::Connect {}).await;
		
		let mut lock = self.server_connection.lock().await;
		*lock = Some(connection);
	}
	
	pub async fn logout() {
	
	}
	
	pub async fn update() {
	
	}
	
	async fn start(self: Arc<Self>) {
		let network_manager  = self.clone();
		tokio::spawn(async {

		});
	}
}

#[async_trait]
impl<M: 'static> IManager for NetworkManager<M>
	where M: From<NetworkManagerMessage> + Send {
	async fn run(self: Arc<Self>) {
		let networkManager = self.clone();
		loop {
			sleep(Duration::new(1,0)).await;
			println!("networkManager tick")
		}
	}
	
	async fn start(self: &Arc<Self>) {
		let network_manager  = self.clone();
		tokio::spawn(
			network_manager.run()
		);
	}
}

#[cfg(test)]
mod test {
	use std::future::Future;
	use std::panic;
	use tokio::sync::mpsc::channel;
	use serverlib::Server;
	use crate::managers::Network::NetworkManagerMessage;
	use crate::managers::Network::NetworkManagerMessage::Info;
	use crate::managers::NetworkManager;
	
	async fn wrap_setup<T,F>(test: T)
		where T: FnOnce(u16) -> F,
					F: Future
	{
		let server = Server::new().unwrap();
		let port = server.port();
		tokio::spawn(
			async move {
				server.start().await;
			}
		);
		
		test(port).await;
	}
	
	#[tokio::test]
	async fn create_network_manager() {
		use NetworkManagerMessage::Info;
		let (tx,rx) =
			channel::<NetworkManagerMessage>(16);
		
		wrap_setup(|port| {
			async move {
				let network = NetworkManager::new(tx);
				let info = network.info(format!("localhost:{}", port)).await.expect("Failed to fetch info");
				assert_eq!(info, Info {
					server_name: "oof".to_string(),
					server_owner: "michael".to_string()
				});
			}
		}).await;
	}
	
	// #[tokio::test]
	// async fn fetch_server_info() {
	// 	wrap_setup(|port| {
	// 		async move {
	//
	// 		}
	// 	})
	// }
}
