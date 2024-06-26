use tokio::task::JoinHandle;

use crate::client::server_writer_connection::ServerWriterConnection;

pub mod network_connection;
pub mod server_reader_connection;
pub mod server_writer_connection;

pub enum NetworkState {
	Disconnected,
	Connection {
		reader_handle: JoinHandle<()>,
		writer: ServerWriterConnection,
	},
}
