use actix::Addr;
use mlua::UserData;
use crate::client_management::Client;

pub(crate) struct ScriptableClient {
	addr: Addr<Client>
}

impl UserData for ScriptableClient {

}

impl From<Addr<Client>> for ScriptableClient {
	fn from(addr: Addr<Client>) -> Self {
		Self {
			addr
		}
	}
}