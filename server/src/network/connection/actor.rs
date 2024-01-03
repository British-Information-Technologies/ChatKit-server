use std::{io::Write, net::SocketAddr, pin::Pin, sync::Arc, time::Duration};

use actix::{
	clock::timeout,
	fut::wrap_future,
	Actor,
	ActorContext,
	Addr,
	AsyncContext,
	Context,
	Handler,
	WeakRecipient,
};
use futures::{future::join_all, Future, FutureExt};
use tokio::{
	io::{split, AsyncBufReadExt, AsyncWriteExt, BufReader, ReadHalf, WriteHalf},
	net::TcpStream,
	sync::Mutex,
};

use super::{ConnectionMessage, ConnectionObservableOutput};
use crate::{
	network::connection::messages::ConnectionPrivateMessage,
	prelude::messages::ObservableMessage,
};

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
	write_half: Arc<Mutex<WriteHalf<TcpStream>>>,
	_address: SocketAddr,
	observers: Vec<WeakRecipient<ConnectionObservableOutput>>,
}

impl Connection {
	/// Creates a new Conneciton actor from a Tokio TcpStream,
	/// and start's its execution.
	/// returns: the Addr of the connection.
	pub(crate) fn new(stream: TcpStream, address: SocketAddr) -> Addr<Self> {
		let (read_half, write_half) = split(stream);
		let addr = Connection {
			write_half: Arc::new(Mutex::new(write_half)),
			_address: address,
			observers: Vec::new(),
		}
		.start();
		addr.do_send(ConnectionPrivateMessage::DoRead(BufReader::new(read_half)));
		addr
	}

	#[inline]
	fn broadcast(
		&self,
		ctx: &mut <Self as Actor>::Context,
		data: ConnectionObservableOutput,
	) {
		let futs: Vec<Pin<Box<dyn Future<Output = ()> + Send>>> = self
			.observers
			.iter()
			.cloned()
			.map(|r| {
				let data = data.clone();
				async move {
					if let Some(r) = r.upgrade() {
						let _ = r.send(data).await;
					}
				}
				.boxed()
			})
			.collect();
		let _ = ctx.spawn(wrap_future(async {
			join_all(futs).await;
		}));
	}

	#[inline]
	fn do_read(
		&mut self,
		ctx: &mut <Self as Actor>::Context,
		mut buf_reader: BufReader<ReadHalf<TcpStream>>,
	) {
		let weak_addr = ctx.address().downgrade();

		let read_fut = async move {
			let dur = Duration::from_millis(100);
			let mut buffer_string: String = Default::default();

			let read_fut = buf_reader.read_line(&mut buffer_string);
			let Ok(Ok(len)) = timeout(dur, read_fut).await else {
				if let Some(addr) = weak_addr.upgrade() {
					addr.do_send(ConnectionPrivateMessage::DoRead(buf_reader));
				}
				return;
			};

			if len == 0 {
				println!("[Connection] readline returned 0");
				if let Some(addr) = weak_addr.upgrade() {
					addr.do_send(ConnectionPrivateMessage::Close);
				}
				return;
			}

			if let Some(addr) = weak_addr.upgrade() {
				let _ = addr
					.send(ConnectionPrivateMessage::Broadcast(
						ConnectionObservableOutput::RecvData(
							addr.downgrade(),
							buffer_string.clone(),
						),
					))
					.await;
			}

			if let Some(addr) = weak_addr.upgrade() {
				addr.do_send(ConnectionPrivateMessage::DoRead(buf_reader));
			}
		};
		ctx.spawn(wrap_future(read_fut));
	}

	fn close_connection(&self, ctx: &mut <Self as Actor>::Context) {
		use ConnectionObservableOutput::ConnectionClosed;
		self.broadcast(ctx, ConnectionClosed(ctx.address().downgrade()))
	}
}

impl Actor for Connection {
	type Context = Context<Self>;

	/// runs when the actor is started.
	/// takes out eh read_half ad turns it into a buffered reader
	/// then eneters loop readling lines from the tcp stream
	fn started(&mut self, _ctx: &mut Self::Context) {
		println!("[Connection] started");
	}

	fn stopped(&mut self, ctx: &mut Self::Context) {
		use ConnectionObservableOutput::ConnectionClosed;
		println!("[Connection] stopped");
		for recp in self.observers.iter() {
			if let Some(recp) = recp.upgrade() {
				recp.do_send(ConnectionClosed(ctx.address().downgrade()))
			}
		}
	}
}

impl Handler<ObservableMessage<ConnectionObservableOutput>> for Connection {
	type Result = ();
	fn handle(
		&mut self,
		msg: ObservableMessage<ConnectionObservableOutput>,
		_ctx: &mut Self::Context,
	) -> <Self as actix::Handler<ObservableMessage<ConnectionObservableOutput>>>::Result{
		use ObservableMessage::{Subscribe, Unsubscribe};
		match msg {
			Subscribe(r) => {
				println!("[Connection] adding subscriber");
				self.observers.push(r);
			}
			Unsubscribe(r) => {
				println!("[Connection] removing subscriber");
				let r = r.upgrade();
				self.observers = self
					.observers
					.clone()
					.into_iter()
					.filter(|a| a.upgrade() != r)
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
		let writer = Arc::downgrade(&self.write_half);

		match msg {
			SendData(d) => {
				ctx.spawn(wrap_future(async move {
					let Some(writer) = writer.upgrade() else {
						return;
					};

					println!("[Connection] sending data");
					let mut lock = writer.lock().await;
					let mut buffer = Vec::new();
					let _ = writeln!(&mut buffer, "{}", d.as_str());
					let _ = lock.write_all(&buffer).await;
				}));
			}
			CloseConnection => ctx.stop(),
		};
	}
}

// impl Handler<SelfMessage> for Connection {
// 	type Result = ();
// 	fn handle(&mut self, msg: SelfMessage, ctx: &mut Self::Context) -> Self::Result {
// 		use ConnectionObservableOutput::RecvData;
// 		use SelfMessage::UpdateObserversWithData;
// 		match msg {
// 			UpdateObserversWithData(data) => {
// 				let send = ctx.address();
// 				let addr = self.address;
// 				// this is a mess
// 				let futs: Vec<Pin<Box<dyn Future<Output = ()> + Send>>> = self
// 					.observers
// 					.iter()
// 					.cloned()
// 					.map(|r| {
// 						let send = send.clone();
// 						let data = data.clone();
// 						async move {
// 							let _ = r.send(RecvData(send, addr, data)).await;
// 						}
// 						.boxed()
// 					})
// 					.collect();
// 				let _ = ctx.spawn(wrap_future(async {
// 					join_all(futs).await;
// 				}));
// 			}
// 		};
// 	}
// }

impl Handler<ConnectionPrivateMessage> for Connection {
	type Result = ();

	fn handle(
		&mut self,
		msg: ConnectionPrivateMessage,
		ctx: &mut Self::Context,
	) -> Self::Result {
		use ConnectionPrivateMessage::Broadcast;
		match msg {
			Broadcast(data) => self.broadcast(ctx, data),
			ConnectionPrivateMessage::DoRead(buf_reader) => {
				self.do_read(ctx, buf_reader)
			}
			ConnectionPrivateMessage::Close => self.close_connection(ctx),
		};
	}
}

impl Drop for Connection {
	fn drop(&mut self) {
		println!("[Connection] Dropping value")
	}
}
