use crate::client_management::client::Client;
use crate::client_management::client::ClientDataResponse::{Username, Uuid};
use crate::client_management::client::{ClientDataMessage, ClientDataResponse};
use actix::Addr;
use mlua::{Error, UserData, UserDataMethods};

#[derive(Clone)]
pub(crate) struct ScriptableClient {
	addr: Addr<Client>,
}

impl UserData for ScriptableClient {
	fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
		methods.add_async_method("username", |_lua, obj, ()| async move {
			let name: Option<ClientDataResponse> =
				obj.addr.send(ClientDataMessage::Username).await.ok();
			if let Some(Username(name)) = name {
				Ok(name)
			} else {
				Err(Error::RuntimeError(
					"Name returned null or other value".to_string(),
				))
			}
		});

		methods.add_async_method("uuid", |_lua, obj, ()| async move {
			let uuid: Option<ClientDataResponse> =
				obj.addr.send(ClientDataMessage::Uuid).await.ok();
			if let Some(Uuid(uuid)) = uuid {
				Ok(uuid.to_string())
			} else {
				Err(Error::RuntimeError(
					"Uuid returned null or other value".to_string(),
				))
			}
		});

		methods.add_async_method("address", |_lua, obj, ()| async move {
			let address: Option<ClientDataResponse> =
				obj.addr.send(ClientDataMessage::Address).await.ok();
			if let Some(Username(address)) = address {
				Ok(address)
			} else {
				Err(Error::RuntimeError(
					"address returned null or other value".to_string(),
				))
			}
		});
	}
}

impl From<Addr<Client>> for ScriptableClient {
	fn from(addr: Addr<Client>) -> Self {
		Self { addr }
	}
}
