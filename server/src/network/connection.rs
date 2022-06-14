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
use actix::SpawnHandle;
use futures::future::join_all;
use futures::Future;
use futures::FutureExt;
use serde::Serialize;
use std::io::Write;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use tokio::io::split;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
use tokio::io::ReadHalf;
use tokio::io::WriteHalf;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

/// This is a message that can be sent to the Connection.
#[derive(Message)]
#[rtype(result = "()")]
pub enum ConnectionMessage {
	SendData(String),
	CloseConnection,
}

#[derive(Message)]
#[rtype(result = "()")]
pub enum ConnectionOuput {
	RecvData(Addr<Connection>, SocketAddr, String),
	ConnectionClosed(Addr<Connection>),
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
pub struct Connection {
	read_half: Option<ReadHalf<TcpStream>>,
	write_half: Arc<Mutex<WriteHalf<TcpStream>>>,
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
			write_half: Arc::new(Mutex::new(write_half)),
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
		println!("[Connection] started");
		let addr = ctx.address();
		let read_half = self
			.read_half
			.take()
			.expect("What the hell did yu do wrong");
		ctx.spawn(wrap_future(async move {
			let mut reader = BufReader::new(read_half);
			let mut buffer_string = String::new();

			while let Ok(len) = reader.read_line(&mut buffer_string).await {
				use SelfMessage::{UpdateObserversWithData};
				use ConnectionMessage::CloseConnection;
				if len == 0 {
					println!("[Connection] connection closed");
					addr.send(CloseConnection)
						.await
						.expect("[Connection] failed to send close message to self");
					return
				}

				println!("[Connection] read line");
				addr
					.send(UpdateObserversWithData(buffer_string.clone()))
					.await;
				buffer_string.clear();
			}
		}));
	}

	fn stopped(&mut self, ctx: &mut Self::Context) {
		use ConnectionOuput::ConnectionClosed;
		println!("[Connection] stopped");
		for recp in self.observers.iter() {
			recp.do_send(ConnectionClosed(ctx.address()));
		}
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
				println!("[Connection] adding subscriber");
				self.observers.push(r);
			}
			Unsubscribe(r) => {
				println!("[Connection] removing subscriber");
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
		ctx: &mut Self::Context,
	) -> Self::Result {
		use ConnectionMessage::{CloseConnection, SendData};
		let writer = self.write_half.clone();

		match msg {
			SendData(d) => {
				ctx.spawn(wrap_future(async move {
					println!("[Connection] sending data");
					let mut lock = writer.lock().await;
					let mut buffer = Vec::new();
					writeln!(&mut buffer, "{}", d.as_str());
					lock.write_all(&buffer).await;
				}));
			}
			CloseConnection => ctx.stop(),
		};
	}
}

impl Handler<SelfMessage> for Connection {
	type Result = ();
	fn handle(
		&mut self,
		msg: SelfMessage,
		ctx: &mut Self::Context,
	) -> Self::Result {
		use ConnectionOuput::RecvData;
		use SelfMessage::UpdateObserversWithData;
		match msg {
			UpdateObserversWithData(data) => {
				let send = ctx.address();
				let addr = self.address.clone();
				// this is a mess
				let futs: Vec<Pin<Box<dyn Future<Output = ()> + Send>>> = self
					.observers
					.iter()
					.cloned()
					.map(|r| {
						let send = send.clone();
						let data = data.clone();
						async move {
							let _ = r.send(RecvData(send, addr, data)).await;
						}
						.boxed()
					})
					.collect();
				let _ = ctx.spawn(wrap_future(async {
					join_all(futs).await;
				}));
			}
		};
	}
}
