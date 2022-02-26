use std::io::{Error, ErrorKind};
use std::io::Write;
use std::mem;
use std::sync::Arc;
use serde::Serialize;
use serde::de::DeserializeOwned;
use tokio::io;
use tokio::io::{AsyncWriteExt, BufReader, AsyncBufReadExt, ReadHalf, WriteHalf};
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct Connection {
	stream_rx: Mutex<Option<BufReader<ReadHalf<tokio::net::TcpStream>>>>,
	stream_tx: Mutex<Option<WriteHalf<tokio::net::TcpStream>>>,
}

impl Connection {
	pub fn new() -> Arc<Self> {
		Arc::new(Connection {
			stream_rx: Mutex::new(None),
			stream_tx: Mutex::new(None),
		})
	}
	
	pub async fn connect<T: ToSocketAddrs>(&self, host: T) -> Result<(), Error> {
		let connection = TcpStream::connect(host).await?;
		let (rd, wd) = io::split(connection);
		
		let mut writer_lock = self.stream_tx.lock().await;
		let mut reader_lock = self.stream_rx.lock().await;
		
		let _ = mem::replace(&mut *writer_lock, Some(wd));
		let _ = mem::replace(&mut *reader_lock, Some(BufReader::new(rd)));
		
		Ok(())
	}
	
	pub async fn write<T>(&self, message: T) -> Result<(), Error>
		where T: Serialize {
		let mut out_buffer = Vec::new();


		let out = serde_json::to_string(&message).unwrap();

		writeln!(&mut out_buffer, "{}", out)?;

		let mut writer_lock = self.stream_tx.lock().await;

		let old = mem::replace(&mut *writer_lock, None);

		return if let Some(mut writer) = old {
			writer.write_all(&out_buffer).await?;
			writer.flush().await?;
			let _ = mem::replace(&mut *writer_lock, Some(writer));
			Ok(())
		} else {
			Err(Error::new(ErrorKind::Interrupted, "Writer does not exist"))
		}
	}
	
	pub async fn read<T>(&self) -> Result<T,Error>
		where T: DeserializeOwned {
		let mut buffer = String::new();
		let mut reader_lock = self.stream_rx.lock().await;
		let old = mem::replace(&mut *reader_lock, None);

		if let Some(mut reader) = old {
			let _ = reader.read_line(&mut buffer).await?;
			let _ = mem::replace(&mut *reader_lock, Some(reader));
			Ok(serde_json::from_str(&buffer).unwrap())
		} else {
			Err(Error::new(ErrorKind::Interrupted, "Reader does not exist"))
		}
	}
}

impl From<TcpStream> for Connection {
	fn from(stream: TcpStream) -> Self {
		let (rd, wd) = io::split(stream);
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
		let server = TcpListener::bind("localhost:0").await?;
		let addr = server.local_addr()?;
		
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
