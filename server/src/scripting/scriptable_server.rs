use crate::scripting::scriptable_client_manager::ScriptableClientManager;
use crate::scripting::scriptable_network_manager::ScriptableNetworkManager;
use actix::Addr;
use mlua::{Error, UserData, UserDataMethods};

use crate::server::ServerDataResponse::{ClientManager, Name, NetworkManager, Owner};
use crate::server::*;

#[derive(Clone)]
pub(crate) struct ScriptableServer {
	pub(super) addr: Addr<Server>,
}

impl UserData for ScriptableServer {
	fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
		methods.add_async_method("name", |_lua, obj, ()| async move {
			let name: Option<ServerDataResponse> =
				obj.addr.send(ServerDataMessage::Name).await.ok();
			if let Some(Name(name)) = name {
				Ok(name)
			} else {
				Err(Error::RuntimeError(
					"Name returned null or other value".to_string(),
				))
			}
		});

		methods.add_async_method("owner", |_lua, obj, ()| async move {
			let owner: Option<ServerDataResponse> =
				obj.addr.send(ServerDataMessage::Owner).await.ok();
			if let Some(Owner(name)) = owner {
				Ok(name)
			} else {
				Err(Error::RuntimeError(
					"Name returned null or other value".to_string(),
				))
			}
		});

		methods.add_async_method("client_manager", |_lua, obj, ()| async move {
			let name: Option<ServerDataResponse> =
				obj.addr.send(ServerDataMessage::ClientManager).await.ok();
			if let Some(ClientManager(Some(cm))) = name {
				Ok(ScriptableClientManager::from(cm))
			} else {
				Err(Error::RuntimeError(
					"Name returned null or other value".to_string(),
				))
			}
		});

		methods.add_async_method("network_manager", |_lua, obj, ()| async move {
			let name: Option<ServerDataResponse> =
				obj.addr.send(ServerDataMessage::NetworkManager).await.ok();
			if let Some(NetworkManager(Some(nm))) = name {
				Ok(ScriptableNetworkManager::from(nm))
			} else {
				Err(Error::RuntimeError(
					"Name returned null or other value".to_string(),
				))
			}
		});
	}
}

impl From<Addr<Server>> for ScriptableServer {
	fn from(addr: Addr<Server>) -> Self {
		Self { addr }
	}
}
