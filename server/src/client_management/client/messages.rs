use crate::client_management::client::client::Client;
use actix::{Message, MessageResponse, WeakAddr};
use foundation::ClientDetails;
use uuid::Uuid;

/// Message sent ot the clients delegate
#[derive(Message)]
#[rtype(result = "()")]
pub enum ClientMessage {
	SendUpdate(Vec<ClientDetails>),
	SendMessage { from: Uuid, content: String },
	SendGlobalMessage { from: Uuid, content: String },
}

#[derive(Message)]
#[rtype(result = "ClientDataResponse")]
pub enum ClientDataMessage {
	Details,
	Uuid,
	Username,
	Address,
}

#[derive(MessageResponse)]
pub enum ClientDataResponse {
	Details(ClientDetails),
	Uuid(Uuid),
	Username(String),
	Address(String),
}

/// message that is sent to all observers of the current client.
#[derive(Message, Clone)]
#[rtype(result = "()")]
pub enum ClientObservableMessage {
	SendMessageRequest(WeakAddr<Client>, Uuid, String),
	SendGlobalMessageRequest(WeakAddr<Client>, String),
	UpdateRequest(WeakAddr<Client>),
}
