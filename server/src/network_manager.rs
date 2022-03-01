use std::io::{Error, ErrorKind};
use std::sync::Arc;

use uuid::Uuid;

use async_trait::async_trait;

use tokio::net::TcpListener;
use tokio::sync::mpsc::Sender;
use tokio::{select};
use tokio::sync::Mutex;

use foundation::connection::Connection;

use foundation::messages::network::{NetworkSockIn, NetworkSockOut};
use foundation::prelude::IManager;

#[derive(Debug)]
pub enum NetworkManagerMessage {
	ClientConnecting {
		uuid: Uuid,
		address: String,
		username: String,

		connection: Arc<Connection>
	},
}

impl PartialEq for NetworkManagerMessage {
	fn eq(&self, other: &Self) -> bool {
		use NetworkManagerMessage::ClientConnecting;

		match (self, other) {
			(ClientConnecting {uuid,address,username, .. },
				ClientConnecting {
					uuid: other_uuid,
					address: other_address,
					username: other_username, ..
				}) => uuid == other_uuid && address == other_address && username == other_username,

			#[allow(unreachable_patterns)]
			_ => false
		}
	}
}

/// # NetworkManager
///
/// This handles all new incoming connections to the server, involved with the chat services.
///
/// ## Fields
///	- address: the socket address that the server is listening on.
///	- listener: the TcpListener that is receiving connections.
/// - out_channel: the channel that will be sent events from NetworkManager.
pub struct NetworkManager<Out>
	where
		Out: From<NetworkManagerMessage> + Send
{
	listener: Mutex<TcpListener>,
	out_channel: Sender<Out>,
}

impl<Out> NetworkManager<Out>
	where
			Out: From<NetworkManagerMessage> + Send
{
	pub async fn new(
		address: &str,
		out_channel: Sender<Out>
	) -> Result<Arc<NetworkManager<Out>>, Error> {

		let listener = TcpListener::bind(address).await?;

		Ok(Arc::new(NetworkManager {
			listener: Mutex::new(listener),
			out_channel,
		}))
	}

	/// This fetches the port from the NetworkManager
	pub async fn port(&self) -> u16 {
		self.listener.lock().await.local_addr().unwrap().port()
	}

	/// This fetches the IP address from the NetworkManager
	#[allow(dead_code)]
	pub async fn address(&self) -> String {
		self.listener.lock().await.local_addr().unwrap().ip().to_string()
	}

	async fn handle_connection(&self, connection: Arc<Connection>) -> Result<(), Error>{
		use NetworkSockIn::{Info, Connect};
		use NetworkSockOut::{GotInfo, Request, Connecting};

		connection.write(Request).await?;

		match connection.read().await? {
			Info => connection.write(GotInfo {
				server_name: "TestServer".into(),
				server_owner: "Michael".into()
			}).await?,
			Connect { uuid, address, username } => {
				connection.write(Connecting).await?;

				let _ = self.out_channel.send(NetworkManagerMessage::ClientConnecting {
					uuid,
					address,
					username,

					connection,
				}.into()).await;
			}
			#[allow(unreachable_patterns)]
			_ => {
				return Err(Error::new(ErrorKind::InvalidData, "Did not receive valid message"));
			}
		}
		Ok(())
	}
}

#[async_trait]
impl<Out: 'static> IManager for NetworkManager<Out>
	where
			Out: From<NetworkManagerMessage> + Send
{
	async fn run(self: &Arc<Self>) {
		let lock = self.listener.lock().await;
		select! {
			val = lock.accept() => {
				if let Ok((stream, _addr)) = val {
					let _ = self.handle_connection(Arc::new(stream.into())).await;
				}
			}
		}
	}
}

#[cfg(test)]
mod test {
	use std::io::Error;
	use tokio::sync::mpsc::channel;
	use uuid::Uuid;
	use foundation::connection::Connection;
	use foundation::messages::network::NetworkSockIn::{Connect, Info};
	use foundation::messages::network::NetworkSockOut;
	use foundation::messages::network::NetworkSockOut::{Connecting, GotInfo, Request};
	use foundation::prelude::IManager;
	use crate::network_manager::{NetworkManager, NetworkManagerMessage::{ClientConnecting}, NetworkManagerMessage};

	#[tokio::test]
	async fn test_network_fetch_info() -> Result<(), Error> {

		let (tx,_rx) = channel::<NetworkManagerMessage>(16);

		let network_manager =
			NetworkManager::new("localhost:0",tx).await?;
		network_manager.start();
		let port = network_manager.port().await;

		let client = Connection::new();
		client.connect(format!("localhost:{}", port)).await?;

		assert_eq!(client.read::<NetworkSockOut>().await?, Request);
		client.write(Info).await?;

		let out = client.read::<NetworkSockOut>().await?;
		assert_eq!(
			out,
			GotInfo {server_owner: "Michael".into(), server_name: "TestServer".into()}
		);

		Ok(())
	}

	#[tokio::test]
	async fn test_network_login() -> Result<(), Error> {
		let (tx, mut rx) = channel::<NetworkManagerMessage>(16);
		let network_manager =
			NetworkManager::new("localhost:0",tx).await?;
		network_manager.start();

		let port = network_manager.port().await;
		let client = Connection::new();
		client.connect(format!("localhost:{}", port)).await?;

		assert_eq!(client.read::<NetworkSockOut>().await?, Request);


		// construct client data
		let uuid = Uuid::new_v4();
		let address = "localhost";
		let username = "TestUser";

		client.write(Connect {
			uuid,
			address: address.to_string(),
			username: username.to_string()
		}).await?;

		let res: NetworkSockOut = client.read().await?;

		assert_eq!(res, Connecting);

		let network_out = rx.recv().await.unwrap();

		assert_eq!(network_out, ClientConnecting {
			uuid,
			address: address.to_string(),
			username: username.to_string(),
			connection: client
		});

		Ok(())
	}
}