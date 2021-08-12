use std::fmt::Debug;
use std::io::Error;
use std::io::Write;
use std::sync::Arc;

use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::io::split;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::ReadHalf;
use tokio::io::WriteHalf;
use tokio::io::{BufReader, BufWriter};
use tokio::net::TcpStream;
use tokio::sync::Mutex;

use crate::prelude::StreamMessageSender;
use crate::prelude::TransformerFn;

type TransformerVec = Vec<TransformerFn>;

pub struct SocketHandler {
	stream_tx: Mutex<WriteHalf<tokio::net::TcpStream>>,
	stream_rx: Mutex<BufReader<ReadHalf<tokio::net::TcpStream>>>,

	send_transformer: Mutex<TransformerVec>,
	recv_transformer: Mutex<TransformerVec>,
}

impl SocketHandler {
	pub fn new(connection: TcpStream) -> Arc<Self> {
		let (rd, wd) = split(connection);
		let reader = BufReader::new(rd);

		Arc::new(SocketHandler {
			stream_tx: Mutex::new(wd),
			stream_rx: Mutex::new(reader),

			send_transformer: Mutex::new(Vec::new()),
			recv_transformer: Mutex::new(Vec::new()),
		})
	}

	pub async fn push_layer(self: &Arc<Self>, send_func: TransformerFn, recv_func: TransformerFn) {
		let mut send_lock = self.send_transformer.lock().await;
		let mut recv_lock = self.recv_transformer.lock().await;
		send_lock.push(send_func);
		recv_lock.push(recv_func);
	}

	pub async fn pop_layer(self: &Arc<Self>) {
		let mut send_lock = self.send_transformer.lock().await;
		let mut recv_lock = self.recv_transformer.lock().await;
		let _ = send_lock.pop();
		let _ = recv_lock.pop();
	}
}

#[async_trait]
impl StreamMessageSender for SocketHandler {
	async fn send<TOutMessage: Serialize + Send>(
		self: &Arc<Self>,
		message: TOutMessage,
	) -> Result<(), Error> {
		let mut out_buffer: Vec<u8> = Vec::new();
		let message_string = serde_json::to_string(&message)?;
		writeln!(out_buffer, "{}", message_string)?;

		println!("[SocketHandler:send] message_before: {:?}", &out_buffer);

		let transformers = self.send_transformer.lock().await;
		let iter = transformers.iter();

		for func in iter {
			out_buffer = (**func)(&out_buffer);
		}

		println!("[SocketHandler:send] message_after: {:?}", &out_buffer);

		let mut lock = self.stream_tx.lock().await;
		lock.write_all(&out_buffer).await?;
		lock.flush().await?;
		Ok(())
	}

	async fn recv<'de, TInMessage: DeserializeOwned + Send>(
		self: &Arc<Self>,
	) -> Result<TInMessage, Error> {
		let mut in_buffer = String::new();
		let mut lock = self.stream_rx.lock().await;
		lock.read_line(&mut in_buffer).await?;

		println!("[SocketHandler:recv] message_before: {:?}", &in_buffer);

		let transformers = self.recv_transformer.lock().await;
		let iter = transformers.iter();

		let mut in_buffer = in_buffer.into_bytes();

		for func in iter {
			in_buffer = (**func)(&in_buffer);
		}

		println!("[SocketHandler:recv] message_after: {:?}", &in_buffer);

		let in_buffer = String::from_utf8(in_buffer).expect("invalid utf_8");

		let message: TInMessage = serde_json::from_str(&in_buffer).unwrap();

		Ok(message)
	}
}

impl Debug for SocketHandler {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		write!(f, "[SocketSender]")
	}
}

#[cfg(test)]
mod test {
	use tokio::runtime::Runtime;
	use std::sync::Once;
	use std::time::Duration;
	
	use tokio::task;
	use tokio::io::{AsyncReadExt, AsyncWriteExt};
	use tokio::net::TcpListener;
	use tokio::net::TcpStream;
	use tokio::time::sleep;
	
	use super::SocketHandler;
	use crate::helpers::start_server;
	use crate::helpers::create_test_shared;
	use crate::prelude::StreamMessageSender;
	use crate::encryption::create_encryption_transformers;


	static SERVER_INIT: Once = Once::new();

	fn setup() {
		SERVER_INIT.call_once(|| {
			std::thread::spawn(|| {
				let rt = Runtime::new().unwrap();
				rt.block_on(start_server())
			
			});
		})
	}

	#[tokio::test]
	async fn test_socket_sender() {
		setup();
		task::spawn(start_server());

		let socket = TcpStream::connect("localhost:5600")
			.await
			.expect("failed to connect");

		let handle = SocketHandler::new(socket);
		let _ = handle.send::<bool>(true).await;
		let message = handle.recv::<bool>().await.unwrap();

		assert!(message);
	}

	#[tokio::test]
	async fn test_socket_sender_with_encryption() {
		setup();
		task::spawn(start_server());

		let socket = TcpStream::connect("localhost:5600")
			.await
			.unwrap();

		let shared = create_test_shared();
		let (en, de) = create_encryption_transformers(shared, b"12345678901234567890123456789011");
		let handle = SocketHandler::new(socket);

		handle.push_layer(en, de).await;

		let _ = handle.send::<bool>(true).await;
		let message = handle.recv::<bool>().await.unwrap();

		assert!(message);
	}
}
