use std::io::Error;
use std::sync::Arc;

use futures::lock::Mutex;
use mlua::{Lua, UserDataFields, UserDataMethods};
use mlua::prelude::LuaUserData;
use tokio::sync::mpsc::{channel, Receiver};
use uuid::Uuid;
use foundation::connection::Connection;
use foundation::prelude::IManager;

use crate::client_manager::{ClientManager, ClientMgrMessage};
use crate::network_manager::{NetworkManager, NetworkManagerMessage};

#[derive(Debug)]
pub enum ServerMessage {
	ClientConnected {
		uuid: Uuid,
		address: String,
		username: String,
		connection: Arc<Connection>
	},
	BroadcastGlobalMessage {from: Uuid, content: String},
}

impl From<NetworkManagerMessage> for ServerMessage {
	fn from(msg: NetworkManagerMessage) -> Self {
		use NetworkManagerMessage::{ClientConnecting};

		match msg {
			ClientConnecting {
				uuid,
				address,
				username,
				connection
			} => ServerMessage::ClientConnected {
				uuid,
				address,
				username,
				connection
			},
			#[allow(unreachable_patterns)]
			_ => unimplemented!()
		}
	}
}

impl From<ClientMgrMessage> for ServerMessage {
	fn from(msg: ClientMgrMessage) -> Self {
		use ClientMgrMessage::{BroadcastGlobalMessage,};

		match msg {
			BroadcastGlobalMessage {
				from,
				content,
			} => ServerMessage::BroadcastGlobalMessage {
				from,
				content
			},
			_ => unimplemented!()
		}
	}
}


/// # Server
/// authors: @michael-bailey, @Mitch161
/// This Represents a server instance.
/// It is composed of a client manager and a network manager.
///
/// # Attributes
/// - client_manager: The servers client manager.
/// - network_manager: The servers network manager.
/// - receiver: The servers channel for communication by managers.
/// - lua: The servers lua context, used for running lua scripts.
///
pub struct Server {
	client_manager: Arc<ClientManager<ServerMessage>>,
	network_manager: Arc<NetworkManager<ServerMessage>>,
	receiver: Mutex<Receiver<ServerMessage>>,
	lua: Lua,
}

impl Server {
	/// Create a new server object
	pub async fn new() -> Result<Arc<Server>, Error> {
		let (
			sender,
			receiver
		) = channel(1024);

		let server = Arc::new(Server {
			client_manager: ClientManager::new(sender.clone()),
			network_manager: NetworkManager::new("0.0.0.0:5600", sender).await?,
			receiver: Mutex::new(receiver),
			lua: Lua::new(),
		});

		server.lua.globals().set("Server", ServerLua(server.clone())).unwrap();

		Ok(server)
	}

	pub async fn port(self: &Arc<Server>) -> u16 {
		self.network_manager.port().await
	}
	
	pub async fn start(self: &Arc<Server>) {
		// start client manager and network manager
		self.network_manager.clone().start();
		self.client_manager.clone().start();

		// clone block items
		let server = self.clone();

		loop {
			let mut lock = server.receiver.lock().await;
			if let Some(message) = lock.recv().await {
				println!("[server]: received message {:?}", &message);

				match message {
					ServerMessage::ClientConnected {
						uuid,
						address,
						username,
						connection
					} => {
						server.client_manager
							.add_client(
								uuid,
								username,
								address,
								connection
							).await
					},
					ServerMessage::BroadcastGlobalMessage {
						from: _,
						content: _,
					} => {
						// server
						// 	.client_manager
						// 	.clone()
						// 	.send_message(
						// 		ClientMgrMessage::BroadcastGlobalMessage {sender, content}
						// 	).await
					}
					#[allow(unreachable_patterns)]
					_ => {unimplemented!()}
				}
			}
		}
	}
}

/// # ServerLua
/// A wrapper struct for making the Server lua scriptable.
///
/// # Attributes
/// - 1: A reference to the server.
struct ServerLua(Arc<Server>);

impl LuaUserData for ServerLua {
	fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
		fields.add_field_method_get("ClientManager", |lua,server| {
			Ok("unimplemented")
		});
		fields.add_field_method_get("NetworkManager", |lua,server| {
			Ok("unimplemented")
		});
	}

	fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(_methods: &mut M) {

	}
}
