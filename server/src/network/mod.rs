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


pub struct SocketSender {
	stream_tx: Mutex<WriteHalf<tokio::net::TcpStream>>,
	stream_rx: Mutex<BufReader<ReadHalf<tokio::net::TcpStream>>>,
}

impl SocketSender {
	pub fn new(connection: TcpStream) -> Arc<Self> {
		let (rd, wd) = split(connection);
		let reader = BufReader::new(rd);

		Arc::new(SocketSender {
			stream_tx: Mutex::new(wd),
			stream_rx: Mutex::new(reader),
		})
	}
}

#[async_trait]
impl StreamMessageSender for SocketSender {
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

impl Debug for SocketSender {
	
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>)
		-> std::result::Result<(), std::fmt::Error> {
			write!(f, "[SocketSender]")
	}
}