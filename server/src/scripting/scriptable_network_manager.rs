use actix::Addr;
use mlua::{Error, UserData, UserDataMethods};

use crate::network::{
	NetworkDataMessage,
	NetworkDataOutput::IsListening,
	NetworkManager,
};

#[derive(Clone)]
pub(crate) struct ScriptableNetworkManager {
	addr: Addr<NetworkManager>,
}

impl UserData for ScriptableNetworkManager {
	fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
		methods.add_async_method("Listening", |_lua, obj, ()| async move {
			let is_listening =
				obj.addr.send(NetworkDataMessage::IsListening).await.ok();
			if let Some(IsListening(is_listening)) = is_listening {
				Ok(is_listening)
			} else {
				Err(Error::RuntimeError(
					"Uuid returned null or other value".to_string(),
				))
			}
		});
	}
}

impl From<Addr<NetworkManager>> for ScriptableNetworkManager {
	fn from(addr: Addr<NetworkManager>) -> Self {
		Self { addr }
	}
}
