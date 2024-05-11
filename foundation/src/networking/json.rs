use std::io;

use serde::{de::DeserializeOwned, Serialize};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

pub async fn write_message<S, M>(stream: &mut S, message: M)
where
	S: AsyncWrite + AsyncWriteExt + Unpin,
	M: Serialize,
{
	let mut message = serde_json::to_string(&message).unwrap();
	message.push('\n');
	_ = stream.write(message.as_bytes()).await;
}

// todo: Handle error properly
pub async fn read_message<S, M>(stream: &mut S) -> io::Result<M>
where
	S: AsyncRead + AsyncReadExt + Unpin,
	M: DeserializeOwned,
{
	let string = read_line(stream).await?;
	Ok(serde_json::from_str(&string).unwrap())
}

#[allow(clippy::redundant_guards, clippy::needless_range_loop)]
async fn read_line<S>(stream: &mut S) -> Result<String, std::io::Error>
where
	S: AsyncRead + AsyncReadExt + Unpin,
{
	let mut buf = vec![0; 1024];
	let mut newline_found = false;
	let mut result = Vec::new();
	loop {
		let n = match stream.read(&mut buf).await {
			Ok(n) if n == 0 => return Ok(String::from_utf8(result).unwrap()),
			Ok(n) => n,
			Err(e) => return Err(e),
		};
		for i in 0..n {
			if buf[i] == b'\n' {
				newline_found = true;
				break;
			}
			result.push(buf[i]);
		}
		if newline_found {
			return Ok(String::from_utf8(result).unwrap());
		}
	}
}
