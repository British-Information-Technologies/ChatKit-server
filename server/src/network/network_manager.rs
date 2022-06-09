//! # network_manager
//! This module contains the network manager actor
//! it's role involves handling new oncomming network connections

use crate::network::listener::ListenerOutput;
use crate::network::Connection;
use crate::network::ConnectionInitiator;
use crate::network::InitiatorOutput;
use crate::network::ListenerMessage;
use crate::network::NetworkListener;
use actix::Actor;
use actix::Addr;
use actix::AsyncContext;
use actix::Context;
use actix::Handler;
use actix::Message;
use actix::WeakRecipient;
use foundation::ClientDetails;

#[derive(Message, Debug, Ord, PartialOrd, Eq, PartialEq)]
#[rtype(result = "()")]
pub(crate) enum NetworkMessage {
	StartListening,
	StopListening,
}

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) enum NetworkOutput {
	NewConnection(Addr<Connection>),
	NewClient(Addr<Connection>, ClientDetails),
	InfoRequested(Addr<Connection>),
}

pub(crate) struct NetworkManager {
	listener_addr: Option<Addr<NetworkListener>>,
	delegate: WeakRecipient<NetworkOutput>,
	initiators: Vec<Addr<ConnectionInitiator>>,
}

impl NetworkManager {
	pub(crate) fn new(
		delegate: WeakRecipient<NetworkOutput>,
	) -> Addr<NetworkManager> {
		NetworkManager {
			listener_addr: None,
			delegate,
			initiators: Vec::new(),
		}
		.start()
	}

	fn start_listener(&mut self, _ctx: &mut <Self as actix::Actor>::Context) {
		use ListenerMessage::StartListening;
		if let Some(addr) = self.listener_addr.as_ref() {
			addr.do_send(StartListening);
		}
	}

	fn stop_listener(&mut self, _ctx: &mut <Self as actix::Actor>::Context) {
		use ListenerMessage::StopListening;
		if let Some(addr) = self.listener_addr.as_ref() {
			addr.do_send(StopListening);
		}
	}

	/// Handles a new connection from the Listener.
	/// This creates a new ConnectionInitaliser.
	/// This completes the first part of the protocol.
	#[inline]
	fn new_connection(
		&mut self,
		ctx: &mut <Self as Actor>::Context,
		connection: Addr<Connection>,
	) {
		println!("[NetworkManager] Got new connection");

		let init = ConnectionInitiator::new(
			ctx.address().recipient().downgrade(),
			connection,
		);
		self.initiators.push(init);
	}

	#[inline]
	fn remove_initiator(&mut self, sender: Addr<ConnectionInitiator>) {
		let index = self.initiators.iter().position(|i| *i == sender).unwrap();
		println!("[NetworkManager] removed initiator at:{}", index);
		self.initiators.remove(index);
	}

	/// handles a initiator client request
	/// this will, forward the conenction and client details
	/// to the server actor to be dispatched to the appropriate
	/// manager
	#[inline]
	#[allow(unreachable_code, unused_variables)]
	fn client_request(
		&mut self,
		_ctx: &mut <Self as Actor>::Context,
		sender: Addr<ConnectionInitiator>,
		_connection: Addr<Connection>,
		_client_details: ClientDetails,
	) {
		println!("[NetworkManager] recieved client request");
		todo!();
		self.remove_initiator(sender);
	}

	/// This sends the connection to the server
	/// which will in turn take over the protocol by sending
	/// the servers infomation.
	#[inline]
	fn info_request(
		&mut self,
		_ctx: &mut <Self as Actor>::Context,
		sender: Addr<ConnectionInitiator>,
		connection: Addr<Connection>,
	) {
		use NetworkOutput::InfoRequested;
		println!("[NetworkManager] Got recieved info request");
		if let Some(delegate) = self.delegate.upgrade() {
			delegate.do_send(InfoRequested(connection));
		}
		self.remove_initiator(sender);
	}
}

impl Actor for NetworkManager {
	type Context = Context<Self>;

	fn started(&mut self, ctx: &mut Self::Context) {
		println!("started network manager");
		let recipient = ctx.address().recipient();
		self
			.listener_addr
			.replace(NetworkListener::new("0.0.0.0:5600", recipient));
	}
}

impl Handler<NetworkMessage> for NetworkManager {
	type Result = ();
	fn handle(
		&mut self,
		msg: NetworkMessage,
		ctx: &mut <Self as actix::Actor>::Context,
	) -> <Self as Handler<NetworkMessage>>::Result {
		use NetworkMessage::{StartListening, StopListening};
		match msg {
			StartListening => self.start_listener(ctx),
			StopListening => self.stop_listener(ctx),
		}
	}
}

impl Handler<ListenerOutput> for NetworkManager {
	type Result = ();
	fn handle(
		&mut self,
		msg: ListenerOutput,
		ctx: &mut Self::Context,
	) -> Self::Result {
		use ListenerOutput::NewConnection;
		match msg {
			NewConnection(connection) => self.new_connection(ctx, connection),
		};
	}
}

impl Handler<InitiatorOutput> for NetworkManager {
	type Result = ();
	fn handle(
		&mut self,
		msg: InitiatorOutput,
		ctx: &mut Self::Context,
	) -> Self::Result {
		use InitiatorOutput::{ClientRequest, InfoRequest};
		match msg {
			ClientRequest(sender, addr, client_details) => {
				self.client_request(ctx, sender, addr, client_details)
			}
			InfoRequest(sender, addr) => self.info_request(ctx, sender, addr),
		}
	}
}
