use std::sync::Arc;
use mlua::prelude::LuaUserData;
use mlua::{UserDataFields, UserDataMethods};
use crate::lua::ClientManagerLua;
use crate::Server;

/// # ServerLua
/// A wrapper struct for making the Server lua scriptable.
///
/// # Attributes
/// - 1: A reference to the server.
#[derive(Clone)]
pub struct ServerLua(Arc<Server>);

impl ServerLua {
	pub fn new(server: Arc<Server>) -> Self {
		ServerLua(server)
	}
}

impl LuaUserData for ServerLua {
	fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
		fields.add_field_method_get("ClientManager", |lua,this| {
			println!("Getting count");
			Ok(ClientManagerLua(this.0.client_manager.clone(), vec![]))
		});
		fields.add_field_method_get("NetworkManager", |lua,this| {
			Ok("unimplemented")
		});
		fields.add_field_method_get("address", |lua,this| {
			Ok("unimplemented")
		});
	}

	fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(_methods: &mut M) {

	}
}