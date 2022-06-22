use actix::Addr;
use foundation::ClientDetails;
use crate::network::Connection;
use actix::Message;

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