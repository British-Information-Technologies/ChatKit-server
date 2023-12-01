use std::{
	io::{Error, ErrorKind},
	mem,
	sync::{atomic::AtomicBool, Arc},
};

use async_trait::async_trait;
use foundation::{
	connection::Connection,
	messages::{
		client::{ClientStreamIn, ClientStreamOut},
		network::{NetworkSockIn, NetworkSockOut},
	},
	prelude::IManager,
};
use tokio::{
	net::ToSocketAddrs,
	sync::{mpsc::Sender, Mutex},
};
use uuid::Uuid;

use crate::managers::NetworkManagerMessage;

pub struct NetworkManager<M>
where
	M: From<NetworkManagerMessage>,
{
	#[allow(unused)]
	server_connection: Mutex<Option<Arc<Connection>>>,

	#[allow(unused)]
	cursive: Sender<M>,

	is_logged_in: AtomicBool,
}

impl<M> NetworkManager<M>
where
	M: From<NetworkManagerMessage>,
{
	pub fn new(sender: Sender<M>) -> Arc<Self> {
		Arc::new(NetworkManager {
			server_connection: Mutex::new(None),
			cursive: sender,
			is_logged_in: AtomicBool::new(false),
		})
	}

	#[allow(unused)]
	pub async fn info<T: ToSocketAddrs>(
		self: &Arc<Self>,
		host: T,
	) -> Result<NetworkManagerMessage, Error> {
		let connection = Connection::new();
		println!("Created connection");
		connection.connect(host).await?;
		let req = connection.read().await?;

		println!("request: {:?}", req);

		if let NetworkSockOut::Request = req {
			connection
				.write::<NetworkSockIn>(NetworkSockIn::Info)
				.await?;
			return Ok(connection.read::<NetworkSockOut>().await?.into());
		} else {
			Err(Error::new(
				ErrorKind::ConnectionAborted,
				"Request not received",
			))
		}
	}

	#[allow(unused)]
	pub async fn login(
		self: &Arc<Self>,
		host: String,
		uuid: Uuid,
		username: String,
		address: String,
	) -> Result<(), Error> {
		let connection = Connection::new();

		let _ = connection.connect(host).await?;

		println!("created connection");

		let req = connection.read().await?;

		println!("read request");

		return if let NetworkSockOut::Request = req {
			println!("got request");

			connection
				.write(NetworkSockIn::Connect {
					username,
					uuid,
					address,
				})
				.await?;
			let res = connection.read().await?;

			// switch over to ClientStreamOut
			if let ClientStreamOut::Connected = res {
				let mut connection_lock = self.server_connection.lock().await;
				let _ = mem::replace(&mut *connection_lock, Some(connection));
				Ok(())
			} else {
				Err(Error::new(
					ErrorKind::ConnectionRefused,
					format!("expected connecting received: {:?}", res),
				))
			}
		} else {
			println!("request not found");
			Err(Error::new(
				ErrorKind::ConnectionAborted,
				"Server did not send request",
			))
		};
	}

	#[allow(unused)]
	pub async fn logout(self: &Arc<Self>) -> Result<(), Error> {
		let mut connection_lock = self.server_connection.lock().await;
		let connection = mem::replace(&mut *connection_lock, None).unwrap();

		connection.write(ClientStreamIn::Disconnect).await?;

		return if let ClientStreamOut::Disconnected = connection.read().await? {
			Ok(())
		} else {
			Err(Error::new(
				ErrorKind::InvalidData,
				"disconnect failed, forcing disconnect",
			))
		};
	}
}

#[async_trait]
impl<M: 'static> IManager for NetworkManager<M>
where
	M: From<NetworkManagerMessage> + Send,
{
	async fn run(self: &Arc<Self>) {
		println!("networkManager tick")
	}
}

#[cfg(test)]
mod test {
	use std::future::Future;

	use serverlib::Server;
	use tokio::sync::mpsc::channel;
	use uuid::Uuid;

	use crate::managers::{network::NetworkManagerMessage, NetworkManager};

	async fn wrap_setup<T, F>(test: T)
	where
		T: FnOnce(u16) -> F,
		F: Future,
	{
		let server = Server::new().await.unwrap();
		let port = server.port().await;

		tokio::spawn(async move {
			server.start().await;
		});
		test(port).await;
	}
	#[tokio::test]
	async fn test_fetch_server_info() {
		use NetworkManagerMessage::Info;
		#[allow(unused)]
		let (tx, rx) = channel::<NetworkManagerMessage>(16);

		wrap_setup(|port| async move {
			let network = NetworkManager::new(tx);
			let info = network
				.info(format!("localhost:{}", port))
				.await
				.expect("Failed to fetch info");
			assert_eq!(
				info,
				Info {
					server_name: "oof".to_string(),
					server_owner: "michael".to_string()
				}
			);
		})
		.await;
	}
	#[tokio::test]
	async fn test_login_and_logout_to_server() {
		#[allow(unused)]
		let (tx, rx) = channel::<NetworkManagerMessage>(16);

		let network = NetworkManager::new(tx);

		println!("created network manger");

		wrap_setup(|port| async move {
			network
				.login(
					format!("localhost:{}", port),
					Uuid::default(),
					"user1".to_string(),
					"localhost".to_string(),
				)
				.await
				.expect("login failed");

			network.logout().await.expect("logout failed");
		})
		.await;
	}
}
