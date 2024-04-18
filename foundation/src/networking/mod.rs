use std::io::{self, ErrorKind};

use prost::{
	bytes::{BufMut, Bytes, BytesMut},
	Message,
};
use tokio::{
	io::{AsyncReadExt, AsyncWriteExt},
	net::TcpStream,
};

pub async fn write_message<T>(
	stream: &mut TcpStream,
	message: T,
) -> io::Result<()>
where
	T: Message + Default,
{
	let message = encode_message::<T>(&message)?;
	stream.write_all(&message).await?;
	Ok(())
}

pub fn encode_message<T>(msg: &T) -> io::Result<Bytes>
where
	T: Message,
{
	let length = msg.encoded_len();
	let mut buffer = BytesMut::with_capacity(4 + length);
	buffer.put_u32(length as u32);
	let encode_result = msg.encode(&mut buffer);
	if let Err(err) = encode_result {
		return Err(io::Error::new(
			ErrorKind::InvalidInput,
			format!("message encoding failed: {:?}", err),
		));
	}

	Ok(buffer.into())
}

pub async fn read_message<T>(stream: &mut TcpStream) -> io::Result<T>
where
	T: Message + Default,
{
	let size = stream.read_u32().await?;
	println!("need to read: {}", size);
	let mut buffer = BytesMut::with_capacity(size as usize);
	unsafe { buffer.set_len(size as usize) };

	println!("buffer size: {}", buffer.len());

	stream.read_exact(&mut buffer).await?;
	println!("buffer size after read: {}", buffer.len());
	println!("buffer content: {:?}", buffer);

	let message = decode_message::<T>(buffer.into())?;
	println!("decoded message: {:?}", message);

	Ok(message)
}

pub fn decode_message<T>(buffer: Bytes) -> io::Result<T>
where
	T: Message + Default,
{
	let msg_result = T::decode(buffer);
	match msg_result {
		Ok(msg) => Ok(msg),
		Err(err) => Err(io::Error::new(
			ErrorKind::InvalidInput,
			format!("message decoding failed: {:?}", err),
		)),
	}
}