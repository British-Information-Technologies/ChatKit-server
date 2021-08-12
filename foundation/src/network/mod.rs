use tokio::io::AsyncWrite;
use tokio::io::AsyncRead;
use tokio::io::AsyncReadExt;
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
use tokio::io::{BufReader};
use tokio::sync::Mutex;

use crate::prelude::StreamMessageSender;
use crate::prelude::TransformerFn;

type TransformerVec = Vec<TransformerFn>;

pub struct SocketHandler<T>
where 
	T: AsyncRead + AsyncWrite + Send
{
	stream_tx: Mutex<WriteHalf<T>>,
	stream_rx: Mutex<BufReader<ReadHalf<T>>>,

	send_transformer: Mutex<TransformerVec>,
	recv_transformer: Mutex<TransformerVec>,
}

impl<T> SocketHandler<T> 
where 
	T: AsyncReadExt + AsyncWriteExt + Send
{
	pub fn new(connection: T) -> Arc<Self> {
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
impl<T> StreamMessageSender for SocketHandler<T>
where 
	T: AsyncReadExt + AsyncWriteExt + Send
{
	async fn send<TOutMessage: Serialize + Send>(
		self: &Arc<Self>,
		message: TOutMessage,
	) -> Result<(), Error> {
		let message_string = serde_json::to_string(&message)?;
		let mut out_buffer = Vec::from(message_string);
		let message_length = out_buffer.len();
		println!("[SocketHandler:send] message_length:{:?}", &message_length);

		println!("[SocketHandler:send] message_before: {:?}", &out_buffer);

		let transformers = self.send_transformer.lock().await;
		let iter = transformers.iter();

		for func in iter {
			let transform = (**func)(&out_buffer);
			out_buffer.clear();
			out_buffer.extend_from_slice(&transform);
		}

		let data = base64::encode(&out_buffer[..message_length]);

		println!("[SocketHandler:send] message_encode_base64: {:?}", &data);

		out_buffer.clear();

		writeln!(out_buffer, "{}", data)?;

		println!("[SocketHandler:send] message_out: {:?}", &out_buffer);

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
		let mut length = lock.read_line(&mut in_buffer).await.unwrap();
		in_buffer.pop();
		println!("[SocketHandler:recv] message_in: {:?}", &in_buffer);
		
		let mut in_buffer = base64::decode(in_buffer).unwrap();
		println!("[SocketHandler:recv] message_decoded_base64: {:?}", &in_buffer);

		length = in_buffer.len();

		let transformers = self.recv_transformer.lock().await;
		let iter = transformers.iter().rev();
		for func in iter {
			let transform = (**func)(&in_buffer);
			in_buffer.clear();
			in_buffer.extend_from_slice(&transform[..length]);
		}
		println!("[SocketHandler:recv] message_after_transoformed: {:?}", &in_buffer);

		let in_buffer = String::from_utf8(in_buffer).unwrap();
		let message: TInMessage = serde_json::from_str(&in_buffer).unwrap();
		Ok(message)
	}
}

impl<T> Debug for SocketHandler<T> 
where 
	T: AsyncReadExt + AsyncWriteExt + Send
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		write!(f, "[SocketSender]")
	}
}

#[cfg(test)]
mod test {
	use crate::helpers::BufferStream;
	use super::SocketHandler;
	
	use crate::prelude::StreamMessageSender;
	use crate::encryption::helpers::create_test_shared;
	use crate::encryption::create_encryption_transformers;

	#[tokio::test]
	async fn test_socket_sender() {

		let stream = BufferStream::new();

		let handle = SocketHandler::new(stream);
		let _ = handle.send::<bool>(true).await.unwrap();
		let message = handle.recv::<bool>().await.unwrap();

		assert!(message);
	}

	#[tokio::test]
	async fn test_socket_sender_with_encryption() {

		let stream = BufferStream::new();

		let shared = create_test_shared();
		let (en, de) = create_encryption_transformers(shared, b"12345678901234567890123456789011");
		let handle = SocketHandler::new(stream);

		handle.push_layer(en, de).await;

		handle.send::<bool>(true).await.unwrap();
		let message = handle.recv::<bool>().await.unwrap();

		assert!(message);
	}
}
