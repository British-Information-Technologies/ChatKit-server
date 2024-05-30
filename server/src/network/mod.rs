use std::{io, net::SocketAddr};

use async_trait::async_trait;
use foundation::prelude::{ClientDetails, GlobalMessage, PrivateMessage};
use tokio::{net::TcpStream, sync::mpsc::UnboundedSender, task::JoinHandle};
use uuid::Uuid;

use crate::{
	connection::connection_manager::ConnectionManagerMessage,
	server_va::ServerMessages,
};

pub mod json;
pub mod protobuf;

pub enum ConnectionType {
	ProtobufConnection(TcpStream, SocketAddr),
	JsonConnection(TcpStream, SocketAddr),
}

#[async_trait]
pub trait NetworkListener {
	async fn new(channel: UnboundedSender<ServerMessages>) -> Self;
	async fn run(&self);
	fn start_run(sender: UnboundedSender<ServerMessages>) -> JoinHandle<()>;
}

#[async_trait::async_trait]
pub trait NetworkConnection: Send {
	async fn get_request(&mut self) -> io::Result<ServerRequest>;
	async fn send_info(self: Box<Self>, name: String, owner: String);
	async fn send_connected(
		self: Box<Self>,
		uuid: Uuid,
	) -> (Box<dyn ClientWriter>, Box<dyn ClientReader>);
}

#[async_trait::async_trait]
pub trait ClientReader: Send {
	fn start_run(
		self: Box<Self>,
		uuid: Uuid,
		channel: UnboundedSender<ConnectionManagerMessage>,
	) -> JoinHandle<()>;
}

#[async_trait::async_trait]
pub trait ClientWriter: Send {
	async fn send_clients(&mut self, clients: Vec<ClientDetails>);
	async fn send_global_messages(&mut self, messages: Vec<GlobalMessage>);
	async fn send_global_message(&mut self, message: GlobalMessage);
	async fn send_private_message(&mut self, message: PrivateMessage);
	async fn send_disconnect(&mut self);
	async fn send_client_joined(&mut self, details: ClientDetails);
	async fn send_client_left(&mut self, uuid: Uuid);
}

pub enum ServerRequest {
	GetInfo,
	Connect {
		username: String,
		uuid: uuid::Uuid,
		addr: SocketAddr,
	},
	Ignore,
}
