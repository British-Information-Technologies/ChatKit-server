use actix::{Message, MessageResponse, WeakAddr};
use foundation::ClientDetails;
use uuid::Uuid;

use crate::client_management::client::Client;

/// Message sent ot the clients delegate
#[derive(Message)]
#[rtype(result = "()")]
pub enum ClientMessage {
	Update(Vec<ClientDetails>),
	Message { from: Uuid, content: String },
	GlobalMessage { from: Uuid, content: String },
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
	Message(WeakAddr<Client>, Uuid, String),
	GlobalMessage(WeakAddr<Client>, String),
	Update(WeakAddr<Client>),
}
