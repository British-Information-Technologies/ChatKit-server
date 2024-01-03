use std::net::SocketAddr;

use actix::{
	Actor,
	ActorContext,
	Addr,
	AsyncContext,
	Context,
	Handler,
	WeakAddr,
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
	network::InitiatorOutput,
	prelude::{
		actors::Connection,
		messages::{
			ConnectionMessage,
			ConnectionObservableOutput,
			ObservableMessage,
		},
	},
};

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
		sender: WeakAddr<Connection>,
		ctx: &mut <Self as Actor>::Context,
		data: String,
	) {
		use InitiatorOutput::{ClientRequest, InfoRequest};
		use NetworkSockIn::{Connect, Info};

		let msg = from_str::<NetworkSockIn>(data.as_str());
		if let Err(e) = msg.as_ref() {
			println!("[ConnectionInitiator] error decoding message {}", e);
			self.error(ctx, sender);
			return;
		}
		let msg = msg.unwrap();

		println!("[ConnectionInitiator] matching request");
		if let (Some(delegate), Some(sender)) =
			(self.delegate.upgrade(), sender.upgrade())
		{
			match msg {
				Info => {
					delegate.do_send(InfoRequest(ctx.address().downgrade(), sender))
				}
				Connect {
					uuid,
					username,
					address,
				} => delegate.do_send(ClientRequest(
					ctx.address().downgrade(),
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
		sender: WeakAddr<Connection>,
	) {
		use ConnectionMessage::{CloseConnection, SendData};
		if let Some(sender) = sender.upgrade() {
			sender.do_send(SendData(
				to_string::<ClientStreamOut>(&Error {
					msg: "Error in connection initiator?".to_owned(),
				})
				.unwrap(),
			));
			sender.do_send(CloseConnection);
		}
		ctx.stop()
	}
}

impl Actor for ConnectionInitiator {
	type Context = Context<Self>;

	/// on start initiate the protocol.
	/// also add self as a subscriber to the connection.
	fn started(&mut self, ctx: &mut Self::Context) {
		use ConnectionMessage::SendData;
		use NetworkSockOut::Request;
		use ObservableMessage::Subscribe;

		println!("[ConnectionInitiator] started");

		self
			.connection
			.do_send(Subscribe(ctx.address().recipient().downgrade()));

		self
			.connection
			.do_send(SendData(to_string(&Request).unwrap()));
	}

	/// once stopped remove self from the connection subscribers
	fn stopped(&mut self, ctx: &mut Self::Context) {
		use ObservableMessage::Unsubscribe;
		println!("[ConnectionInitiator] stopped");
		self
			.connection
			.do_send(Unsubscribe(ctx.address().recipient().downgrade()));
	}
}

impl Handler<ConnectionObservableOutput> for ConnectionInitiator {
	type Result = ();
	fn handle(
		&mut self,
		msg: ConnectionObservableOutput,
		ctx: &mut Self::Context,
	) -> Self::Result {
		use ConnectionObservableOutput::RecvData;

		if let RecvData(sender, data) = msg {
			self.handle_request(sender, ctx, data)
		}
	}
}

impl Drop for ConnectionInitiator {
	fn drop(&mut self) {
		println!("[ConnectionInitiator] Dropping value")
	}
}
