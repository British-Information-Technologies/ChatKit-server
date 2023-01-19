use std::net::SocketAddr;

use actix::{Message, WeakAddr};

use crate::prelude::actors::Connection;

/// This is a message that can be sent to the Connection.
#[derive(Message)]
#[rtype(result = "()")]
pub(crate) enum ConnectionMessage {
	SendData(String),
	CloseConnection,
}

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub(crate) enum ConnectionObservableOutput {
	RecvData(WeakAddr<Connection>, SocketAddr, String),
	ConnectionClosed(WeakAddr<Connection>),
}

#[derive(Message)]
#[rtype(result = "()")]
pub(super) enum ConnectionPrivateMessage {
	Broadcast(ConnectionObservableOutput),
}
