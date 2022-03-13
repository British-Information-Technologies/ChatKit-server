use std::io::Error;
use std::sync::Arc;

use uuid::Uuid;

use tokio::sync::{Mutex, mpsc::{channel, Receiver}};
use tokio::fs::{create_dir, DirBuilder, File, read_dir};

use mlua::{Lua, UserDataFields, UserDataMethods};
use mlua::prelude::LuaUserData;
use tokio::io::AsyncReadExt;
use tokio::join;

use foundation::connection::Connection;
use foundation::prelude::IManager;

use crate::client_manager::{ClientManager, ClientMgrMessage};
use crate::lua::ServerLua;
use crate::network_manager::{NetworkManager, NetworkManagerMessage};
use crate::plugin_manager::PluginManager;

#[derive(Debug,Clone)]
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
	pub client_manager: Arc<ClientManager<ServerMessage>>,
	network_manager: Arc<NetworkManager<ServerMessage>>,
	plugin_manager: Arc<PluginManager>,
	receiver: Mutex<Receiver<ServerMessage>>,
	lua: Arc<Mutex<Lua>>,
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
			plugin_manager: PluginManager::new(),
			receiver: Mutex::new(receiver),
			lua: Arc::new(Mutex::new(Lua::new())),
		});

		server.lua.lock().await.globals().set("Server", ServerLua::new(server.clone())).unwrap();

		server.load_scripts().await?;

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

	pub async fn load_scripts(self: &Arc<Server>) -> Result<(), Error>{
		if let Ok( mut scripts) = read_dir("./scripts").await {
			while let Some(child) = scripts.next_entry().await? {
				let metadata = child.metadata().await?;

				if metadata.is_file() && child.path().extension().unwrap() == "lua" {
					let mut file = File::open(child.path()).await.unwrap();
					let mut data = String::new();
					file.read_to_string(&mut data).await.unwrap();
					let server = self.clone();
					println!("---| loaded script |---\n{}", data);
					println!("---| script output |---");
					server.clone().lua.lock().await.load(&data).exec().unwrap();
				}
			}
		} else {
			create_dir("./scripts").await?;
		}
		Ok(())
	}
}
