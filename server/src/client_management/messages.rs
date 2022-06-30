use actix::{Message, MessageResponse, Addr, WeakAddr};
use uuid::Uuid;
use crate::client_management::{Client, ClientManager};

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) enum ClientManagerMessage {
	AddClient(Uuid, Addr<Client>),
	RemoveClient(Uuid),
}

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) enum ClientManagerOutput {
	UpdateRequest(Addr<ClientManager>),
}

#[derive(Message)]
#[rtype(result = "ClientManagerDataResponse")]
pub enum ClientManagerDataMessage {
	ClientCount,
	Clients
}

#[derive(MessageResponse)]
pub enum ClientManagerDataResponse {
	ClientCount(usize),
	Clients(Vec<WeakAddr<Client>>)
}