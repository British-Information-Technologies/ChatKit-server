use actix::{Message as ActixMessage, MessageResponse};
use uuid::Uuid;

use super::Message;

#[derive(ActixMessage, Debug)]
#[rtype(result = "()")]
pub enum ChatManagerMessage {
	AddMessage(Uuid, String),
}

#[derive(ActixMessage, Debug)]
#[rtype(result = "ChatManagerDataResponse")]
pub enum ChatManagerDataMessage {
	GetMessages,
	GetMessage(usize),
}

#[derive(MessageResponse)]
pub enum ChatManagerDataResponse {
	GotMessages(Vec<Message>),
	GotMessage(Option<Message>),
}
