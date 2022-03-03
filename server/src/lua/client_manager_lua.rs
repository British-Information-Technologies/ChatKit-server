use std::str::FromStr;
use std::sync::Arc;
use mlua::{Function, MetaMethod, Nil, ToLua, UserDataFields, UserDataMethods};
use mlua::prelude::LuaUserData;
use uuid::Uuid;
use crate::client_manager::{ClientManager, ClientMgrMessage};
use crate::lua::ClientLua;

#[derive(Clone)]
pub struct ClientManagerLua<'lua, Out: 'static>(pub Arc<ClientManager<Out>>, pub Vec<Function<'lua>>)
	where
		Out: From<ClientMgrMessage> + Send;

impl<Out: 'static> ClientManagerLua<'_, Out>
	where
		Out: From<ClientMgrMessage> + Send
{
	pub fn new(manager: Arc<ClientManager<Out>>) -> Self {
		ClientManagerLua(manager, Vec::new())
	}
}

impl<Out: 'static> LuaUserData for ClientManagerLua<'_, Out>
	where
		Out: From<ClientMgrMessage> + Clone + Send
{
	fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {

	}

	fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
		methods.add_async_method("getCount", |_lua,this,()| {
			let this = this.0.clone();
			async move {
				Ok(this.clients.lock().await.len())
			}
		});

		methods.add_async_method("getClientList", |_lua,this,()| {
			let this = this.0.clone();
			async move {
				let clients = this.clients.lock().await;
				let clients: Vec<ClientLua<ClientMgrMessage>> = clients.iter()
					.map(|(_id,c)| ClientLua::new(c.clone()))
					.collect();
				Ok(clients)
			}
		});

		methods.add_async_meta_method(MetaMethod::Index, |lua, this, (index): (String)| {
			let manager = this.0.clone();
			async move {
				if let Ok(id) = Uuid::from_str(&index) {
					let map = manager.clients.lock().await;
					if let Some(found) = map.get(&id) {
						return Ok(ClientLua::new(found.clone()).to_lua(lua)?);
					}
				}
				return Ok(Nil);
			}
		});
	}
}