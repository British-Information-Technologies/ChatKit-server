/// # listener.rs
/// An actor for listening for new connections from the network
use crate::network::connection::Connection;
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
use std::sync::Arc;
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
	Started,
	StartFailed,
	NewConnection(Addr<Connection>),
	NoConnection,
	Error,
	Stopped,
}

pub(super) struct NetworkListener {
	address: SocketAddr,
	// delegate: Arc<RwLock<Recipient<ListenerOutput>>>,
	looper: Option<SpawnHandle>,
}

impl NetworkListener {
	pub(crate) fn new<T: ToSocketAddrs>(
		address: T,
		// delegate: Recipient<ListenerOutput>,
	) -> Addr<NetworkListener> {
		NetworkListener {
			address: address
				.to_socket_addrs()
				.unwrap()
				.collect::<Vec<SocketAddr>>()[0],
			// delegate: Arc::new(RwLock::new(delegate)),
			looper: None,
		}
		.start()
	}

	fn start_listening(&mut self, ctx: &mut <Self as Actor>::Context) {
		let addr = self.address.clone();
		let loop_future = ctx.spawn(wrap_future(async move {
			let listener = TcpListener::bind(addr).await.unwrap();
			while let Ok((stream, addr)) = listener.accept().await {
				let conn = Connection::new(stream, addr);
			}
		}));
	}
	fn stop_listening(&mut self, ctx: &mut <Self as Actor>::Context) {
		if let Some(fut) = self.looper.take() {
			ctx.cancel_future(fut);
		}
	}
}

impl Actor for NetworkListener {
	type Context = Context<Self>;

	fn started(&mut self, ctx: &mut Self::Context) {}
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
