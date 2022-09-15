use crate::client_management::ClientManagerDataResponse::Clients;
use crate::client_management::{ClientManager, ClientManagerDataMessage};
use crate::scripting::scriptable_client::ScriptableClient;
use actix::Addr;
use mlua::{Error, UserData, UserDataMethods};

#[derive(Clone)]
pub(crate) struct ScriptableClientManager {
	addr: Addr<ClientManager>,
}

impl UserData for ScriptableClientManager {
	fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
		methods.add_async_method("clients", |_lua, obj, ()| async move {
			let res = obj.addr.send(ClientManagerDataMessage::Clients).await;
			if let Ok(Clients(clients)) = res {
				let clients: Vec<ScriptableClient> = clients
					.into_iter()
					.map(|a| a.upgrade())
					.filter(|o| o.is_some())
					.map(|o| o.unwrap())
					.map(|a| ScriptableClient::from(a))
					.collect();

				Ok(clients)
			} else {
				Err(Error::RuntimeError(
					"clients returned null or other value".to_string(),
				))
			}
		})
	}
}

impl From<Addr<ClientManager>> for ScriptableClientManager {
	fn from(addr: Addr<ClientManager>) -> Self {
		Self { addr }
	}
}
