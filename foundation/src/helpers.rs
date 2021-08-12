use std::pin::Pin;
use std::io::Error;
use std::task::Poll;
use std::sync::Mutex;
use std::task::Context;

use tokio::io::ReadBuf;
use tokio::io::{AsyncRead, AsyncWrite};

pub struct BufferStream {
	buffer: Mutex<Vec<u8>>,
}

impl BufferStream {
	pub fn new() -> BufferStream {
		BufferStream {
			buffer: Mutex::new(Vec::new()),
		}
	}
}

impl AsyncRead for BufferStream {
	fn poll_read(
		self: Pin<&mut Self>, 
    _cx: &mut Context, 
		buf: &mut ReadBuf<'_>
	) -> Poll<Result<(), futures::io::Error>> {
		let mut lock = self.buffer.lock().unwrap();

		let a = if buf.remaining() < lock.len() {buf.remaining()} else {lock.len()};

		buf.put_slice(&lock[..a]);

		*lock = Vec::from(&lock[a..]);

		Poll::Ready(Ok(()))
	}
}

impl AsyncWrite for BufferStream {
	fn poll_write(
		self: Pin<&mut Self>, 
		_cx: &mut Context<'_>, 
		buf: &[u8]
	) -> Poll<Result<usize, Error>> {
		let mut lock = self.buffer.lock().unwrap();
		lock.extend_from_slice(buf);
		Poll::Ready(Ok(buf.len()))
	}

	fn poll_flush(
			self: Pin<&mut Self>, 
			_cx: &mut Context<'_>
	) -> Poll<Result<(), Error>> {
		Poll::Ready(Ok(()))
	}

	fn poll_shutdown(
			self: Pin<&mut Self>, 
			_cx: &mut Context<'_>
	) -> Poll<Result<(), Error>> {
		Poll::Ready(Ok(()))
	}
}

#[cfg(test)]
mod test {
	
	use tokio::io::split;
	use tokio::io::AsyncWriteExt;

	use crate::helpers::BufferStream;
	use tokio::io::AsyncReadExt;

	#[tokio::test]
	async fn test_reading_and_writing() {
		let stream = BufferStream::new();

		let (mut rd, mut wd) = split(stream);

		let _ = wd.write_all(b"1010").await;

		let mut buf: [u8; 4] = [0; 4];

		let _ = rd.read(&mut buf[..]).await;

		println!("[test_reading_and_writing] {:?}", &buf[..]);

		assert_eq!(b"1010", &buf[..]);
	}


	#[tokio::test]
	async fn test_reading_small() {
		let stream = BufferStream::new();

		let (mut rd, mut wd) = split(stream);

		let _ = wd.write_all(b"10100101").await;

		let mut buf: [u8; 4] = [0; 4];

		let _ = rd.read(&mut buf[..]).await;

		println!("[test_reading_and_writing] {:?}", &buf[..]);

		assert_eq!(b"1010", &buf[..]);

		let _ = rd.read(&mut buf[..]).await;

		println!("[test_reading_and_writing] {:?}", &buf[..]);

		assert_eq!(b"0101", &buf[..]);
	}
}