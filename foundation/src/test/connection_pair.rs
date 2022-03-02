use std::io::{Error};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::join;
use tokio::net::{TcpStream,TcpListener};
use crate::connection::Connection;

pub async fn create_connection_pair()
	-> Result<(Arc<Connection>, (Arc<Connection>, SocketAddr )), Error> {
	let listener: TcpListener = TcpListener::bind("localhost:0000").await?;

	let port = listener.local_addr()?.port();

	let (server_res,client_res) = join!(
		async { TcpStream::connect(format!("localhost:{}", port)).await },
		async { listener.accept().await }
	);

	let (client,addr) = client_res?;
	let server = Arc::new(Connection::from(server_res?));
	let client = Arc::new(Connection::from(client));
	Ok((server,(client,addr)))
}