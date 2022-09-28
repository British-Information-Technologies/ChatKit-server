use actix::{Message, MessageResponse, WeakAddr};
use foundation::{models::message::Message as StoredMessage, ClientDetails};
use uuid::Uuid;

use crate::client_management::client::Client;

/// Message sent ot the clients delegate
#[derive(Message)]
#[rtype(result = "()")]
pub enum ClientMessage {
	ClientList(Vec<ClientDetails>),
	MessageList(Vec<StoredMessage>),

	ClientlySentMessage { from: Uuid, content: String },
	GloballySentMessage { from: Uuid, content: String },
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
	GetClients(WeakAddr<Client>),
	GetGlobalMessages(WeakAddr<Client>),
}
