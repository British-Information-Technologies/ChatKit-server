use foundation::{messages::network::NetworkSockOut, ClientDetails};

#[derive(Debug)]
pub enum NetworkManagerMessage {
	#[allow(unused)]
	Users(Vec<ClientDetails>),
	Info {
		server_name: String,
		server_owner: String,
	},
	Error(&'static str),
}

impl From<NetworkSockOut> for NetworkManagerMessage {
	fn from(other: NetworkSockOut) -> Self {
		use NetworkManagerMessage::{Error, Info as NewInfo};
		use NetworkSockOut::GotInfo as OldInfo;
		match other {
			OldInfo {
				server_name,
				server_owner,
			} => NewInfo {
				server_name,
				server_owner,
			},
			_ => Error("Error occurred with conversion"),
		}
	}
}

impl PartialEq for NetworkManagerMessage {
	fn eq(&self, other: &Self) -> bool {
		use NetworkManagerMessage::Info;
		match self {
			Info {
				server_owner,
				server_name,
			} => {
				if let Info {
					server_owner: other_owner,
					server_name: other_name,
				} = other
				{
					return server_owner == other_owner
						&& server_name == other_name;
				}
				false
			}
			_ => false,
		}
	}
}
