use actix::Addr;
use mlua::UserData;
use crate::client_management::ClientManager;

pub(crate) struct ScriptableClientManager {
	addr: Addr<ClientManager>
}

impl UserData for ScriptableClientManager {

}

impl From<Addr<ClientManager>> for ScriptableClientManager {
	fn from(addr: Addr<ClientManager>) -> Self {
		Self {
			addr
		}
	}
}