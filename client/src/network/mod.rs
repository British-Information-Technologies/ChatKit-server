use crate::network::network_connection::NetworkConnection;

pub mod network_connection;
pub mod server_reader_connection;
pub mod server_writer_connection;

pub enum NetworkState {
	Disconnected,
	ConnectedNetwork(NetworkConnection),
}
