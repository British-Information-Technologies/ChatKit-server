use std::sync::Arc;
use std::io::Write;
use std::io::Error;
use std::fmt::Debug;

use async_trait::async_trait;
use serde::Serialize;
use serde::de::DeserializeOwned;
use tokio::io::split;
use tokio::sync::Mutex;
use tokio::io::ReadHalf;
use tokio::io::BufReader;
use tokio::io::WriteHalf;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncBufReadExt;

use crate::prelude::StreamMessageSender;

type TransformerVec = Vec<fn(&[u8]) -> &[u8]>;

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

	pub async fn push_layer(
		self: &Arc<Self>,
		send_func: fn(&[u8]) -> &[u8],
		recv_func: fn(&[u8]) -> &[u8],
	) {
		let mut send_lock = self.send_transformer.lock().await;
		let mut recv_lock = self.recv_transformer.lock().await;
		send_lock.push(send_func);
		recv_lock.reverse();
		recv_lock.push(recv_func);
		recv_lock.reverse();
	}

	pub async fn pop_layer(self: &Arc<Self>,) {
		let mut send_lock = self.send_transformer.lock().await;
		let mut recv_lock = self.recv_transformer.lock().await;

		let _ = send_lock.pop();

		recv_lock.reverse();
		let _ = recv_lock.pop();
		recv_lock.reverse();
	}
}

#[async_trait]
impl StreamMessageSender for SocketHandler {
	async fn send<TOutMessage: Serialize + Send>
		(self: &Arc<Self>, message: TOutMessage) -> Result<(), Error>
	{ 
		let mut out_buffer: Vec<u8> = Vec::new();
		let message_string = serde_json::to_string(&message)?;
		writeln!(out_buffer, "{}", message_string)?;
		let mut lock = self.stream_tx.lock().await;
		lock.write_all(&out_buffer).await?;
		lock.flush().await?;
		Ok(())
	}

	async fn recv<'de, TInMessage: DeserializeOwned + Send>
		(self: &Arc<Self>) -> Result<TInMessage, Error>
	{ 
		let mut in_buffer = String::new();
		let mut lock = self.stream_rx.lock().await;
		lock.read_line(&mut in_buffer).await?;
		let message: TInMessage = serde_json::from_str(&in_buffer)
			.expect("[StreamMessageSender:recv] deserialisation failed");

		Ok(message)
	}
}

impl Debug for SocketHandler {
	
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>)
		-> std::result::Result<(), std::fmt::Error> {
			write!(f, "[SocketSender]")
	}
}