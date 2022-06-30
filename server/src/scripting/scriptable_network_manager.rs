use actix::Addr;
use mlua::UserData;
use crate::network::NetworkManager;

pub(crate) struct ScriptableNetworkManager {
	addr: Addr<NetworkManager>
}

impl UserData for ScriptableNetworkManager {

}

impl From<Addr<NetworkManager>> for ScriptableNetworkManager {
	fn from(addr: Addr<NetworkManager>) -> Self {
		Self {
			addr
		}
	}
}