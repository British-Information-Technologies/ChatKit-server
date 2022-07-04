use actix::{ActorStreamExt, Addr};
use mlua::{Error, UserData, UserDataFields, UserDataMethods};
use crate::client_management::{ClientManager, ClientManagerDataMessage};
use crate::client_management::ClientManagerDataResponse::Clients;
use crate::scripting::scriptable_client::ScriptableClient;

#[derive(Clone)]
pub(crate) struct ScriptableClientManager {
	addr: Addr<ClientManager>
}

impl UserData for ScriptableClientManager {
	fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
		methods.add_async_method("clients", |lua, obj, ()| async move {
			let res = obj.addr.send(ClientManagerDataMessage::Clients).await;
			if let Ok(Clients(clients)) = res {

				let clients: Vec<ScriptableClient> = clients.into_iter()
					.map(|a| a.upgrade())
					.filter(|o| o.is_some())
					.map(|o| o.unwrap())
					.map(|a| ScriptableClient::from(a))
					.collect();

				Ok(clients)
			} else {
				Err(Error::RuntimeError("clients returned null or other value".to_string()))
			}
		})
	}
}

impl From<Addr<ClientManager>> for ScriptableClientManager {
	fn from(addr: Addr<ClientManager>) -> Self {
		Self {
			addr
		}
	}
}