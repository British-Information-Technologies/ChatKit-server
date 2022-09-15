use crate::client_management::ClientManager;
use crate::network::NetworkManager;
use actix::{Addr, Message, MessageResponse};

#[derive(Message, Clone)]
#[rtype(result = "ServerDataResponse")]
pub enum ServerDataMessage {
	Name,
	Owner,
	ClientManager,
	NetworkManager,
}

#[derive(MessageResponse, Clone)]
pub enum ServerDataResponse {
	Name(String),
	Port(u16),
	Owner(String),
	ClientManager(Option<Addr<ClientManager>>),
	NetworkManager(Option<Addr<NetworkManager>>),
}
