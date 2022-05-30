use crate::network::connection::Connection;
use crate::network::ConnectionInitiator;
use crate::network::InitiatorOutput;
use actix::fut::wrap_future;
use actix::Actor;
use actix::Addr;
use actix::AsyncContext;
use actix::Context;
use actix::Handler;
use actix::Message;
use actix::Recipient;
use actix::SpawnHandle;
use std::net::SocketAddr;
use std::net::ToSocketAddrs;
use tokio::net::TcpListener;
use tokio::sync::RwLock;

#[derive(Message)]
#[rtype(result = "()")]
pub(super) enum ListenerMessage {
	StartListening,
	StopListening,
}

#[derive(Message)]
#[rtype(result = "()")]
pub(super) enum ListenerOutput {
	NewConnection(Addr<Connection>),
	InfoRequest(Addr<Connection>),
}

pub(super) struct NetworkListener {
	address: SocketAddr,
	delegate: Recipient<ListenerOutput>,
	looper: Option<SpawnHandle>,
}

impl NetworkListener {
	pub(crate) fn new<T: ToSocketAddrs>(
		address: T,
		delegate: Recipient<ListenerOutput>,
	) -> Addr<NetworkListener> {
		NetworkListener {
			address: address
				.to_socket_addrs()
				.unwrap()
				.collect::<Vec<SocketAddr>>()[0],
			delegate,
			looper: None,
		}
		.start()
	}

	/// called when the actor is to start listening
	fn start_listening(&mut self, ctx: &mut <Self as Actor>::Context) {
		println!("Network listener started listening");
		let addr = self.address.clone();
		let self_addr = ctx.address();
		let loop_future = ctx.spawn(wrap_future(async move {
			let listener = TcpListener::bind(addr).await.unwrap();
			while let Ok((stream, addr)) = listener.accept().await {
				println!("new tcp connection");
				let conn = Connection::new(stream, addr);
				let connection_initiator =
					ConnectionInitiator::new(self_addr.clone().recipient(), conn);
			}
		}));
	}

	/// called when the actor is to stop listening
	fn stop_listening(&mut self, ctx: &mut <Self as Actor>::Context) {
		println!("Network listener stopped listening");
		if let Some(fut) = self.looper.take() {
			ctx.cancel_future(fut);
		}
	}
}

impl Actor for NetworkListener {
	type Context = Context<Self>;

	fn started(&mut self, ctx: &mut Self::Context) {
		println!("Network listener started");
	}
}

impl Handler<ListenerMessage> for NetworkListener {
	type Result = ();
	fn handle(
		&mut self,
		msg: ListenerMessage,
		ctx: &mut <Self as actix::Actor>::Context,
	) -> Self::Result {
		use ListenerMessage::{StartListening, StopListening};
		match msg {
			StartListening => self.start_listening(ctx),
			StopListening => self.stop_listening(ctx),
		}
	}
}

impl Handler<InitiatorOutput> for NetworkListener {
	type Result = ();
	fn handle(
		&mut self,
		msg: InitiatorOutput,
		ctx: &mut Self::Context,
	) -> Self::Result {
		use InitiatorOutput::{ClientRequest, InfoRequest};
		match msg {
			ClientRequest(addr, client_details) => {}
			InfoRequest(addr) => {
				println!("Got Info request");
				self.delegate.do_send(ListenerOutput::InfoRequest(addr));
			}
		}
	}
}
