use std::cmp::Ordering;
use std::io::Error;
use std::sync::Arc;
use futures::executor::block_on;

use serde::{Deserialize, Serialize};

use uuid::Uuid;

use async_trait::async_trait;

use tokio::{select, task};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::{Mutex};

use foundation::messages::client::{ClientStreamIn, ClientStreamOut};
use foundation::ClientDetails;
use foundation::connection::Connection;
use foundation::messages::client::ClientStreamOut::{Connected, Disconnected};
use foundation::prelude::IManager;

use crate::messages::{ClientMessage};

#[derive(Serialize, Deserialize)]
enum ClientOutMessage {
	MessageTo,
	UpdateRequest,
}

/// # Client
/// This struct represents a connected user.
///
/// ## Attributes
/// - details: store of the clients infomation.
///
/// - stream: The socket for the connected client.
/// - stream_reader: the buffered reader used to receive messages
/// - stream_writer: the buffered writer used to send messages
/// - owner: An optional reference to the owning object.
#[derive(Debug)]
pub struct Client<Out: 'static>
	where
		Out: From<ClientMessage> + Send{
	pub details: ClientDetails,

	// server send channel
	out_channel: Sender<Out>,

	// object channels
	tx: Sender<ClientMessage>,
	rx: Mutex<Receiver<ClientMessage>>,

	connection: Arc<Connection>,
}

// client function implementations
impl<Out> Client<Out>
	where
		Out: From<ClientMessage> + Send {
	pub fn new(
		uuid: Uuid,
		username: String,
		address: String,
		out_channel: Sender<Out>,
		connection: Arc<Connection>
	) -> Arc<Client<Out>> {
		let (sender, receiver) = channel(1024);

		Arc::new(Client {
			details: ClientDetails {
				uuid,
				username,
				address: address.to_string(),
				public_key: None,
			},

			tx: sender,
			rx: Mutex::new(receiver),

			connection: connection,
			out_channel,
		})
	}

	async fn handle_connection(&self, value: Result<ClientStreamIn, Error>) {
		match value {
			Ok(ClientStreamIn::Disconnect) => {
				println!(
					"[Client {:?}]: Disconnect received",
					self.details.uuid
				);
				self.disconnect();
				return;
			}
			_ => {
				println!(
					"[Client {:?}]: command not found",
					self.details.uuid
				);
				let _ = self.out_channel
					.send(ClientMessage::Error.into())
					.await;
			}
		}
	}


	async fn handle_channel(&self, value: Option<ClientMessage>) {
		unimplemented!();
	}

	async fn disconnect(&self) {
		let _ = self.out_channel
			.send(ClientMessage::NewDisconnect {
				id: self.details.uuid,
				connection: self.connection.clone()}.into()
			);
	}

	#[deprecated]
	pub async fn send_message(self: &Arc<Client<Out>>, msg: ClientMessage) {
		let _ = self.tx.send(msg).await;
	}
}

#[async_trait]
impl<Out> IManager for Client<Out>
	where
		Out: From<ClientMessage> + Send
{
	async fn init(self: &Arc<Self>)
		where
			Self: Send + Sync + 'static
	{
		self.connection.write(Connected).await;
	}

	async fn run(self: &Arc<Self>) {

		let mut channel_lock = self.rx.lock().await;

		select! {
			val = self.connection.read::<ClientStreamIn>() => {
				self.handle_connection(val).await;
			}

			val = channel_lock.recv() => {
				self.handle_channel(val).await;
			}
		}
	}
}

// MARK: - use to handle disconnecting
impl<Out> Drop for Client<Out>
	where
		Out: From<ClientMessage> + Send
{
	fn drop(&mut self) {
		let connection = self.connection.clone();
		let out = self.out_channel.clone();
		let id = self.details.uuid.clone();

		tokio::spawn(async move {
			let _ = connection.write(Disconnected).await;
			let _ = out.send(
				ClientMessage::NewDisconnect {
					id,
					connection
				}.into()).await;
		});
	}
}

// MARK: - used for sorting.
impl<Out> PartialEq for Client<Out>
	where
		Out: From<ClientMessage> + Send
{
	fn eq(&self, other: &Self) -> bool {
		self.details.uuid == other.details.uuid
	}
}

impl<Out> Eq for Client<Out>
	where
		Out: From<ClientMessage> + Send
{}

impl<Out> PartialOrd for Client<Out>
	where
		Out: From<ClientMessage> + Send
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl<Out> Ord for Client<Out>
	where
		Out: From<ClientMessage> + Send
{
	fn cmp(&self, other: &Self) -> Ordering {
		self.details.uuid.cmp(&other.details.uuid)
	}
}

#[cfg(test)]
mod test {
	use std::io::Error;
	use tokio::sync::mpsc::channel;
	use uuid::Uuid;
	use foundation::connection::Connection;
	use foundation::messages::client::ClientStreamOut;
	use foundation::messages::client::ClientStreamOut::{Connected, Disconnected};
	use foundation::prelude::IManager;
	use foundation::test::create_connection_pair;
	use crate::client::{Client};
	use crate::messages::ClientMessage;
	use crate::messages::ClientMessage::NewDisconnect;

	#[tokio::test]
	async fn create_client_and_drop() -> Result<(), Error> {
		let (sender, mut receiver) =
			channel::<ClientMessage>(1024);
		let (server, (client_conn, addr)) =
			create_connection_pair().await?;

		// client details
		let uuid = Uuid::new_v4();
		let username = "TestUser".to_string();

		let client = Client::new(
			uuid,
			username,
			addr.to_string(),
			sender.clone(),
			server
		);

		client.start();

		let res = client_conn.read::<ClientStreamOut>().await?;
		assert_eq!(res, Connected);

		drop(client);

		let res = client_conn.read::<ClientStreamOut>().await?;
		assert_eq!(res, Disconnected);

		// fetch from out_channel
		let disconnect_msg = receiver.recv().await.unwrap();
		assert_eq!(disconnect_msg, NewDisconnect {id: uuid, connection: Connection::new()});

		Ok(())
	}
}