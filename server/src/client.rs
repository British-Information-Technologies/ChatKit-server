use std::cmp::Ordering;
use std::io::Error;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use uuid::Uuid;

use async_trait::async_trait;

use tokio::select;
use tokio::sync::mpsc::{channel, Receiver, Sender};

use foundation::messages::client::{ClientStreamIn, ClientStreamOut};
use foundation::ClientDetails;
use foundation::connection::Connection;
use foundation::messages::client::ClientStreamOut::{Connected, Disconnected};
use foundation::prelude::IManager;

use crate::messages::{ClientMessage};

/// # ClientInMessage
///
/// Messages that are sent internally
/// when functions are called on the client
#[derive(Serialize, Deserialize)]
enum ClientInMessage {
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
		Out: From<ClientMessage> + Send
{
	pub details: ClientDetails,
	out_channel: Sender<Out>,
	connection: Arc<Connection>,
}

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
		Arc::new(Client {
			details: ClientDetails {
				uuid,
				username,
				address: address.to_string(),
				public_key: None,
			},
			connection,
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
				self.disconnect().await;
				return;
			}
			Ok(ClientStreamIn::SendGlobalMessage { content }) => {
				let _ = self.out_channel.send(
					ClientMessage::IncomingGlobalMessage {from: self.details.uuid, content}.into()
				).await;
			}
			_ => {
				self.error("Command not found").await;
			}
		}
	}

	pub async fn broadcast_message(&self, from: Uuid, content: String) -> Result<(), Error> {
		self.connection.write(ClientStreamOut::GlobalMessage { from, content }).await?;
		Ok(())
	}

	async fn disconnect(&self) {
		let _ = self.out_channel
			.send(ClientMessage::Disconnect {
				id: self.details.uuid,
			}.into()).await;
	}

	async fn error(&self, msg: &str) {
		let _ = self.connection.write(ClientStreamOut::Error).await;
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
		let _ = self.connection.write(Connected).await;
	}

	async fn run(self: &Arc<Self>) {
		select! {
			val = self.connection.read::<ClientStreamIn>() => {
				self.handle_connection(val).await;
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

		let id = self.details.uuid.clone();

		tokio::spawn(async move {
			let _ = connection.write(Disconnected).await;
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
	use crate::messages::ClientMessage::Disconnect;

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
		assert_eq!(disconnect_msg, Disconnect {id: uuid, connection: Connection::new()});

		Ok(())
	}
}