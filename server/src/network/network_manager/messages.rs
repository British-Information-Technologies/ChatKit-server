use actix::{Addr, Message, MessageResponse};
use foundation::ClientDetails;

use crate::network::Connection;

#[derive(Message, Debug, Ord, PartialOrd, Eq, PartialEq)]
#[rtype(result = "()")]
pub enum NetworkMessage {
	StartListening,
	StopListening,
}

#[derive(Message)]
#[rtype(result = "()")]
pub enum NetworkOutput {
	NewClient(Addr<Connection>, ClientDetails),
	InfoRequested(Addr<Connection>),
}

#[derive(Message, Debug, Ord, PartialOrd, Eq, PartialEq)]
#[rtype(result = "NetworkDataOutput")]
pub enum NetworkDataMessage {
	IsListening,
}

#[derive(MessageResponse)]
pub enum NetworkDataOutput {
	IsListening(bool),
}
