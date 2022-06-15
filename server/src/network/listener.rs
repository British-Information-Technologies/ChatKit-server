use std::net::{SocketAddr, ToSocketAddrs};

use actix::{
	fut::wrap_future,
	Actor,
	Addr,
	AsyncContext,
	Context,
	Handler,
	Message,
	Recipient,
	SpawnHandle,
};
use tokio::net::TcpListener;

use crate::network::{
	connection::Connection,
	ConnectionInitiator,
	InitiatorOutput,
};

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
		println!("[NetworkListener] started listening");
		let addr = self.address.clone();
		let self_addr = ctx.address();
		let delegate = self.delegate.clone();
		let loop_future = ctx.spawn(wrap_future(async move {
			use ListenerOutput::NewConnection;
			let listener = TcpListener::bind(addr).await.unwrap();
			while let Ok((stream, addr)) = listener.accept().await {
				println!("[NetworkListener] accepted socket");
				let conn = Connection::new(stream, addr);
				delegate.do_send(NewConnection(conn));
			}
		}));
	}

	/// called when the actor is to stop listening
	fn stop_listening(&mut self, ctx: &mut <Self as Actor>::Context) {
		println!("[NetworkListener] stopped listening");
		if let Some(fut) = self.looper.take() {
			ctx.cancel_future(fut);
		}
	}
}

impl Actor for NetworkListener {
	type Context = Context<Self>;

	fn started(&mut self, _ctx: &mut Self::Context) {
		println!("[NetworkListener] started");
	}

	fn stopped(&mut self, _ctx: &mut Self::Context) {
		println!("[NetworkListener] stopped");
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
