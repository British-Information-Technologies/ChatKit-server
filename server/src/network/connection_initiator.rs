use crate::network::connection::ConnectionOuput;
use crate::network::Connection;
use crate::prelude::ObservableMessage;
use actix::fut::wrap_future;
use actix::Actor;
use actix::ActorContext;
use actix::Addr;
use actix::AsyncContext;
use actix::Context;
use actix::Handler;
use actix::Message;
use actix::Recipient;
use foundation::messages::network::{NetworkSockIn, NetworkSockOut};
use foundation::ClientDetails;
use serde_json::{from_str, to_string};
use std::net::SocketAddr;

#[derive(Debug, Clone, Copy)]
enum ConnectionPhase {
	Started,
	Requested,
}

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) enum InitiatorOutput {
	InfoRequest(Addr<Connection>),
	ClientRequest(Addr<Connection>, ClientDetails),
}

/// # ConnectionInitiator
/// Handles the initiatin of a new connection.
///
/// This will do one of two things:
/// - Create a new client and send it to the network manager.
/// - Request the eserver info and send it to the connection.
pub(crate) struct ConnectionInitiator {
	delegate: Recipient<InitiatorOutput>,
	connection: Addr<Connection>,
}

impl ConnectionInitiator {
	pub(crate) fn new(
		delegate: Recipient<InitiatorOutput>,
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

		let msg = from_str::<NetworkSockIn>(data.as_str())
			.expect("error deserialising incomming message");

		match msg {
			Info => self
				.delegate
				.do_send(InfoRequest(sender))
				.expect("Failed to send info request Message"),
			Connect {
				uuid,
				username,
				address,
			} => self
				.delegate
				.do_send(ClientRequest(
					sender,
					ClientDetails {
						uuid,
						username,
						address,
						public_key: None,
					},
				))
				.expect("Failed to send connect request"),
		};
		ctx.stop();
	}
}

impl Actor for ConnectionInitiator {
	type Context = Context<Self>;

	/// on start initiate the protocol.
	/// also add self as a subscriber to the connection.
	fn started(&mut self, ctx: &mut Self::Context) {
		use super::ConnectionMessage::SendData;
		use NetworkSockOut::Request;
		use ObservableMessage::Subscribe;

		self
			.connection
			.do_send(Subscribe(ctx.address().recipient()));

		self
			.connection
			.do_send(SendData(to_string(&Request).unwrap()));
	}

	/// once stopped remove self from the connection subscribers
	fn stopped(&mut self, ctx: &mut Self::Context) {
		use ObservableMessage::Unsubscribe;
		self
			.connection
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
		match msg {
			RecvData(sender, addr, data) => {
				self.handle_request(sender, ctx, addr, data)
			}
			ConnectionClosed(_) => todo!(),
		}
	}
}
