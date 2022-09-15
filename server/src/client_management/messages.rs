use crate::client_management::client::Client;
use crate::client_management::ClientManager;
use actix::{Addr, Message, MessageResponse, WeakAddr};
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) enum ClientManagerMessage {
	AddClient(Uuid, Addr<Client>),
	#[allow(dead_code)]
	RemoveClient(Uuid),
}

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) enum ClientManagerOutput {
	#[allow(dead_code)]
	UpdateRequest(Addr<ClientManager>),
}

#[derive(Message)]
#[rtype(result = "ClientManagerDataResponse")]
pub enum ClientManagerDataMessage {
	ClientCount,
	Clients,
}

#[derive(MessageResponse)]
pub enum ClientManagerDataResponse {
	ClientCount(usize),
	Clients(Vec<WeakAddr<Client>>),
}
