use actix::{Message, MessageResponse};
use uuid::Uuid;

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub enum ChatManagerMessage {
	AddMessage(Uuid, String),
}

#[derive(Message, Debug)]
#[rtype(result = "ChatManagerDataResponse")]
pub enum ChatManagerDataMessage {}

#[derive(MessageResponse)]
pub enum ChatManagerDataResponse {}
