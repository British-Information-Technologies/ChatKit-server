use std::net::SocketAddr;

use actix::{
	Actor,
	ActorContext,
	Addr,
	AsyncContext,
	Context,
	Handler,
	Message,
	Recipient,
	WeakRecipient,
};
use foundation::{
	messages::{
		client::{ClientStreamOut, ClientStreamOut::Error},
		network::{NetworkSockIn, NetworkSockOut},
	},
	ClientDetails,
};
use serde_json::{from_str, to_string};

use crate::{
	network::{connection::ConnectionOuput, Connection, ConnectionMessage},
	prelude::messages::ObservableMessage,
};

#[derive(Debug, Clone, Copy)]
enum ConnectionPhase {
	Started,
	Requested,
}

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) enum InitiatorOutput {
	InfoRequest(Addr<ConnectionInitiator>, Addr<Connection>),
	ClientRequest(Addr<ConnectionInitiator>, Addr<Connection>, ClientDetails),
}

/// # ConnectionInitiator
/// Handles the initiatin of a new connection.
///
/// This will do one of two things:
/// - Create a new client and send it to the network manager.
/// - Request the eserver info and send it to the connection.
pub struct ConnectionInitiator {
	delegate: WeakRecipient<InitiatorOutput>,
	connection: Addr<Connection>,
}

impl ConnectionInitiator {
	pub(crate) fn new(
		delegate: WeakRecipient<InitiatorOutput>,
		connection: Addr<Connection>,
	) -> Addr<Self> {
		ConnectionInitiator {
			connection,
			delegate,
		}
		.start()
	}

	fn handle_request(
		&mut self,
		sender: Addr<Connection>,
		ctx: &mut <Self as Actor>::Context,
		address: SocketAddr,
		data: String,
	) {
		use InitiatorOutput::{ClientRequest, InfoRequest};
		use NetworkSockIn::{Connect, Info};
		use NetworkSockOut::{Connecting, GotInfo};
		use ObservableMessage::Unsubscribe;

		let msg = from_str::<NetworkSockIn>(data.as_str());
		if let Err(e) = msg.as_ref() {
			println!("[ConnectionInitiator] error decoding message {}", e);
			self.error(ctx, sender);
			return;
		}
		let msg = msg.unwrap();

		println!("[ConnectionInitiator] matching request");
		if let Some(delegate) = self.delegate.upgrade() {
			match msg {
				Info => delegate.do_send(InfoRequest(ctx.address(), sender)),
				Connect {
					uuid,
					username,
					address,
				} => delegate.do_send(ClientRequest(
					ctx.address(),
					sender,
					ClientDetails {
						uuid,
						username,
						address,
						public_key: None,
					},
				)),
			};
			ctx.stop();
		}
	}

	fn error(
		&mut self,
		ctx: &mut <Self as Actor>::Context,
		sender: Addr<Connection>,
	) {
		use ConnectionMessage::{CloseConnection, SendData};
		sender.do_send(SendData(
			to_string::<ClientStreamOut>(&Error)
				.expect("failed to convert error to string"),
		));
		sender.do_send(CloseConnection);
		ctx.stop()
	}
}

impl Actor for ConnectionInitiator {
	type Context = Context<Self>;

	/// on start initiate the protocol.
	/// also add self as a subscriber to the connection.
	fn started(&mut self, ctx: &mut Self::Context) {
		use NetworkSockOut::Request;
		use ObservableMessage::Subscribe;

		use super::ConnectionMessage::SendData;

		println!("[ConnectionInitiator] started");

		self.connection
			.do_send(Subscribe(ctx.address().recipient()));

		self.connection
			.do_send(SendData(to_string(&Request).unwrap()));
	}

	/// once stopped remove self from the connection subscribers
	fn stopped(&mut self, ctx: &mut Self::Context) {
		use ObservableMessage::Unsubscribe;
		println!("[ConnectionInitiator] stopped");
		self.connection
			.do_send(Unsubscribe(ctx.address().recipient()));
	}
}

impl Handler<ConnectionOuput> for ConnectionInitiator {
	type Result = ();
	fn handle(
		&mut self,
		msg: ConnectionOuput,
		ctx: &mut Self::Context,
	) -> Self::Result {
		use ConnectionOuput::{ConnectionClosed, RecvData};
		use ConnectionPhase::Requested;
		if let RecvData(sender, addr, data) = msg {
			self.handle_request(sender, ctx, addr, data)
		}
	}
}
