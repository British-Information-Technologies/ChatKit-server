use crate::network::connection::ConnectionOuput;
use crate::network::listener::ListenerOutput;
use crate::network::Connection;
use crate::network::ListenerMessage;
use crate::network::NetworkListener;
use crate::prelude::ObservableMessage;
use actix::fut::wrap_future;
use actix::Actor;
use actix::Addr;
use actix::AsyncContext;
use actix::Context;
use actix::Handler;
use actix::Message;
use actix::Recipient;
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
	NewClient(Addr<Connection>, ClientDetails),
	InfoRequested(Addr<Connection>),
}

pub(crate) struct NetworkManager {
	listener_addr: Option<Addr<NetworkListener>>,
	delegate: Recipient<NetworkOutput>,
}

impl NetworkManager {
	pub(crate) fn new(
		delegate: Recipient<NetworkOutput>,
	) -> Addr<NetworkManager> {
		NetworkManager {
			listener_addr: None,
			delegate,
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

	fn new_connection(
		&mut self,
		ctx: &mut <Self as Actor>::Context,
		connection: Addr<Connection>,
	) {
		println!("Got new connection");
		let delegate = self.delegate.clone();
		ctx.spawn(wrap_future(async move {
			// delegate.send(NewConnection(recipient))
			// delegate.send().await;
			// delegate.send().await;
		}));
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
		use ListenerOutput::{InfoRequest, NewConnection};
		match msg {
			NewConnection(connection) => self.new_connection(ctx, connection),
			InfoRequest(connection) => self
				.delegate
				.do_send(NetworkOutput::InfoRequested(connection))
				.expect("failed to send message"),
			_ => todo!(),
		};
	}
}
