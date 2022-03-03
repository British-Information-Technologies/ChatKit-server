use std::sync::Arc;
use mlua::prelude::LuaUserData;
use mlua::{UserDataFields, UserDataMethods};
use crate::client::Client;
use crate::messages::ClientMessage;

pub struct ClientLua<Out: 'static>(pub Arc<Client<Out>>)
	where
		Out: From<ClientMessage> + Send;

impl<Out> ClientLua<Out>
	where
		Out: From<ClientMessage> + Send
{
	pub fn new(client: Arc<Client<Out>>) -> Self {
		ClientLua(client)
	}
}

impl<Out: 'static> LuaUserData for ClientLua<Out>
	where
		Out: From<ClientMessage> + Send
{
	fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
		fields.add_field_method_get("uuid", |_lua, this| {
			Ok(this.0.details.uuid.to_string())
		});

		fields.add_field_method_get("username", |_lua, this| {
			Ok(this.0.details.username.to_string())
		});

		fields.add_field_method_get("address", |_lua, this| {
			Ok(this.0.details.address.to_string())
		});
	}

	fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(_methods: &mut M) {

	}
}