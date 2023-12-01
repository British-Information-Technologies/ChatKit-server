use actix::WeakAddr;
use mlua::{Error, UserData, UserDataMethods};

use crate::server::{ServerDataResponse::Name, *};

#[derive(Clone)]
pub(crate) struct ScriptableServer {
	pub(super) addr: WeakAddr<Server>,
}

impl UserData for ScriptableServer {
	fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
		methods.add_async_method("name", |_lua, obj, ()| async move {
			let Some(send_fut) = obj.addr.upgrade().map(|addr| addr.send(ServerDataMessage::Name)) else {
				return Err(Error::RuntimeError(
					"[ScriptableServer:name] Server doesn't exist. Dunno how you got here".to_string(),
				))
			};

			let name: Option<ServerDataResponse> = send_fut.await.ok();

			let Some(Name(name)) = name else {
				return Err(Error::RuntimeError(
					"[ScriptableServer:name] Name returned nil".to_string(),
				))
			};

			Ok(name)
		});

		methods.add_async_method("owner", |_lua, obj, ()| async move {
			let Some(send_fut) = obj.addr.upgrade().map(|addr| addr.send(ServerDataMessage::Owner)) else {
				return Err(Error::RuntimeError(
					"[ScriptableServer:owner] Server doesn't exist. Dunno how you got here".to_string(),
				))
			};

			let owner: Option<ServerDataResponse> = send_fut.await.ok();

			let Some(Name(owner)) = owner else {
				return Err(Error::RuntimeError(
					"[ScriptableServer:owner] Owner returned nil".to_string(),
				))
			};

			Ok(owner)
		});
	}
}

impl From<WeakAddr<Server>> for ScriptableServer {
	fn from(addr: WeakAddr<Server>) -> Self {
		Self { addr }
	}
}
