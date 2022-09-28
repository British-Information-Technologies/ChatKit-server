use actix::{Message as ActixMessage, MessageResponse};
use foundation::models::message::Message;
use uuid::Uuid;

#[derive(ActixMessage, Debug)]
#[rtype(result = "()")]
pub enum ChatManagerMessage {
	AddMessage(Uuid, String),
}

#[allow(dead_code)]
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
