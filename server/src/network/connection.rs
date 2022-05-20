/// # connection.rs
/// An actor that handles a TcpStream.
use crate::prelude::ObservableMessage;
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
use tokio::io::split;
use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;
use tokio::io::ReadHalf;
use tokio::io::WriteHalf;
use tokio::net::TcpStream;

#[derive(Message)]
#[rtype(result = "()")]
enum ConnectionMessage {
	SendData(String),
	CloseConnection,
}

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) enum ConnectionOuput {
	RecvData(String),
	NoMessage,
}

#[derive(Message)]
#[rtype(result = "()")]
enum SelfMessage {
	UpdateObserversWithData(String),
}

/// # Connection
/// This manages a TcpStream for a given connection.
///
/// ## Fields
/// - read_half: A temporary store fr the read half of the connection.
/// - write_half: The write half of the connection.
/// - address: The socket address of the conneciton.
/// - observers: A list of observers to events created by the connection.
/// - loop_future: the future holding the receiving loop.
pub(crate) struct Connection {
	read_half: Option<ReadHalf<TcpStream>>,
	write_half: WriteHalf<TcpStream>,
	address: SocketAddr,
	observers: Vec<Recipient<ConnectionOuput>>,
	loop_future: Option<SpawnHandle>,
}

impl Connection {
	/// Creates a new Conneciton actor from a Tokio TcpStream,
	/// and start's its execution.
	/// returns: the Addr of the connection.
	pub(super) fn new(stream: TcpStream, address: SocketAddr) -> Addr<Self> {
		let (read_half, write_half) = split(stream);
		Connection {
			read_half: Some(read_half),
			write_half,
			address,
			observers: Vec::new(),
			loop_future: None,
		}
		.start()
	}
}

impl Actor for Connection {
	type Context = Context<Self>;

	/// runs when the actor is started.
	/// takes out eh read_half ad turns it into a buffered reader
	/// then eneters loop readling lines from the tcp stream
	fn started(&mut self, ctx: &mut Self::Context) {
		let addr = ctx.address();
		let read_half = self
			.read_half
			.take()
			.expect("What the hell did yu do wrong");
		ctx.spawn(wrap_future(async move {
			let mut reader = BufReader::new(read_half);
			let mut buffer_string = String::new();

			while let Ok(_) = reader.read_line(&mut buffer_string).await {
				use SelfMessage::UpdateObserversWithData;
				addr
					.send(UpdateObserversWithData(buffer_string.clone()))
					.await;
				buffer_string.clear();
			}
		}));
	}
}

impl Handler<ObservableMessage<ConnectionOuput>> for Connection {
	type Result = ();
	fn handle(
		&mut self,
		msg: ObservableMessage<ConnectionOuput>,
		_ctx: &mut Self::Context,
	) -> <Self as actix::Handler<ObservableMessage<ConnectionOuput>>>::Result {
		use ObservableMessage::{Subscribe, Unsubscribe};
		match msg {
			Subscribe(r) => {
				self.observers.push(r);
			}
			Unsubscribe(r) => {
				self.observers = self
					.observers
					.clone()
					.into_iter()
					.filter(|a| a != &r)
					.collect();
			}
		};
	}
}

impl Handler<ConnectionMessage> for Connection {
	type Result = ();
	fn handle(
		&mut self,
		msg: ConnectionMessage,
		_ctx: &mut Self::Context,
	) -> Self::Result {
		use ConnectionMessage::{CloseConnection, SendData};
		match msg {
			SendData(d) => {}
			CloseConnection => {}
		};
	}
}

impl Handler<SelfMessage> for Connection {
	type Result = ();
	fn handle(
		&mut self,
		msg: SelfMessage,
		_ctx: &mut Self::Context,
	) -> Self::Result {
		use ConnectionOuput::RecvData;
		use SelfMessage::UpdateObserversWithData;
		match msg {
			UpdateObserversWithData(data) => {
				for o in self.observers.clone() {
					o.do_send(RecvData(data.clone()));
				}
			}
		};
	}
}
