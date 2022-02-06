use std::io::Error;
use std::io::Write;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use tokio::io;
use tokio::io::{AsyncWriteExt, BufReader, AsyncBufReadExt, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use crate::messages::client::ClientStreamOut;
use crate::messages::network::NetworkSockIn;

pub struct Connection {
	stream_rx: Mutex<Option<BufReader<ReadHalf<tokio::net::TcpStream>>>>,
	stream_tx: Mutex<Option<WriteHalf<tokio::net::TcpStream>>>,
}

impl Connection {
	pub fn new() -> Self {
		Connection {
			stream_rx: Mutex::new(None),
			stream_tx: Mutex::new(None),
		}
	}
	
	pub async fn connect(&self, host: String) -> Result<(), Error> {
		let connection = TcpStream::connect(host).await?;
		let (rd, mut wd) = io::split(connection);
		*self.stream_tx.lock().await = Some(wd);
		*self.stream_rx.lock().await = Some(BufReader::new(rd));
		Ok(())
	}
	
	pub async fn write<T>(&self, message: T) -> Result<(), Error>
		where T: Serialize {
		let mut out_buffer = Vec::new();
		let out = serde_json::to_string(&message).unwrap();
		writeln!(out_buffer, "{}", out)?;
		let mut writer_lock = self.stream_tx.lock().await;
		let writer = writer_lock.as_mut().unwrap();
		let _ = writer.write_all(&out_buffer).await;
		let _ = writer.flush().await;
		Ok(())
	}
	
	pub async fn read<T>(&self) -> Result<T,Error>
		where T: DeserializeOwned {
		let mut buffer = String::new();
		let mut reader_lock = self.stream_rx.lock().await;
		let reader = reader_lock.as_mut().unwrap();
		reader.read_line(&mut buffer).await?;
		let a: T = serde_json::from_str(&buffer).unwrap();
		Ok(a)
	}
}

impl From<TcpStream> for Connection {
	fn from(stream: TcpStream) -> Self {
		let (rd, mut wd) = io::split(stream);
		Connection {
			stream_tx: Mutex::new(Some(wd)),
			stream_rx: Mutex::new(Some(BufReader::new(rd))),
		}
	}
}

#[cfg(test)]
mod test {
	use std::future::Future;
	use std::io::Error;
	use std::panic;
	use tokio::net::TcpListener;
	use serde::{Serialize,Deserialize};
	use crate::connection::Connection;
	
	#[derive(Serialize, Deserialize, Debug, PartialEq)]
	enum TestMessages {
		Ping,
		Pong
	}
	
	
	#[tokio::test]
	async fn a() -> Result<(), Error> {
		wrap_setup(|port| {
			async move {
				println!("{}", port);
				let connection = Connection::new();
				connection.connect(format!("localhost:{}", &port)).await.unwrap();
				connection.write(&TestMessages::Ping).await.unwrap();
				let res = connection.read::<TestMessages>().await.unwrap();
				
				assert_eq!(res, TestMessages::Pong);
			}
		}).await
	}
	
	
	async fn wrap_setup<T,F>(test: T) -> Result<(), std::io::Error>
		where T: FnOnce(u16) -> F + panic::UnwindSafe,
					F: Future
	{
		let mut server = TcpListener::bind("localhost:0").await?;
		let mut addr = server.local_addr()?;
		
		// create tokio server execution
		tokio::spawn(async move {
			while let Ok((stream, addr)) = server.accept().await {
				use TestMessages::{Ping,Pong};
				
				println!("[server]: Connected {}", &addr);
				let connection = Connection::from(stream);
				if let Ok(Ping) = connection.read::<TestMessages>().await {
					connection.write::<TestMessages>(Pong).await.unwrap()
				}
			}
		});
		
		test(addr.port()).await;
		Ok(())
	}
}
