use crate::config_manager::ConfigManager;
use crate::network::listener::NetworkListener;
use crate::network::listener::{ListenerMessage, ListenerOutput};
use crate::network::network_manager::config::Config;
use crate::network::network_manager::messages::{
	NetworkMessage, NetworkOutput,
};
use crate::network::network_manager::Builder;
use crate::network::{
	Connection, ConnectionInitiator, InitiatorOutput, NetworkDataMessage,
	NetworkDataOutput,
};
use actix::{
	Actor, Addr, AsyncContext, Context, Handler, WeakAddr, WeakRecipient,
};
use foundation::ClientDetails;

/// # NetworkManager
/// this struct will handle all networking functionality.
///
pub struct NetworkManager {
	config: Config,
	config_manager: WeakAddr<ConfigManager>,
	listener_addr: Option<Addr<NetworkListener>>,
	delegate: WeakRecipient<NetworkOutput>,
	initiators: Vec<Addr<ConnectionInitiator>>,
}

impl NetworkManager {
	pub fn new(delegate: WeakRecipient<NetworkOutput>) -> Addr<NetworkManager> {
		NetworkManager {
			config: Config { port: 5600 },
			listener_addr: None,
			delegate,
			initiators: Vec::new(),
			config_manager: ConfigManager::shared().downgrade(),
		}
		.start()
	}

	pub fn create(delegate: WeakRecipient<NetworkOutput>) -> Builder {
		Builder::new(delegate)
	}

	fn start_listener(&mut self, _ctx: &mut <Self as actix::Actor>::Context) {
		use ListenerMessage::StartListening;
		if let Some(addr) = self.listener_addr.as_ref() {
			addr.do_send(StartListening);
		}
	}

	fn stop_listener(&mut self, _ctx: &mut <Self as actix::Actor>::Context) {
		use ListenerMessage::StopListening;
		if let Some(addr) = self.listener_addr.as_ref() {
			addr.do_send(StopListening);
		}
	}

	/// Handles a new connection from the Listener.
	/// This creates a new ConnectionInitaliser.
	/// This completes the first part of the protocol.
	#[inline]
	fn new_connection(
		&mut self,
		ctx: &mut <Self as Actor>::Context,
		connection: Addr<Connection>,
	) {
		println!("[NetworkManager] Got new connection");

		let init = ConnectionInitiator::new(
			ctx.address().recipient().downgrade(),
			connection,
		);
		self.initiators.push(init);
	}

	#[inline]
	fn remove_initiator(&mut self, sender: Addr<ConnectionInitiator>) {
		let index = self.initiators.iter().position(|i| *i == sender).unwrap();
		println!("[NetworkManager] removed initiator at:{}", index);
		self.initiators.remove(index);
	}

	/// handles a initiator client request
	/// this will, forward the conenction and client details
	/// to the server actor to be dispatched to the appropriate
	/// manager
	#[inline]
	fn client_request(
		&mut self,
		_ctx: &mut <Self as Actor>::Context,
		sender: Addr<ConnectionInitiator>,
		connection: Addr<Connection>,
		client_details: ClientDetails,
	) {
		use NetworkOutput::NewClient;
		println!("[NetworkManager] recieved client request");
		if let Some(delegate) = self.delegate.upgrade() {
			delegate.do_send(NewClient(connection, client_details));
		}
		self.remove_initiator(sender);
	}

	/// This sends the connection to the server
	/// which will in turn take over the protocol by sending
	/// the servers infomation.
	#[inline]
	fn info_request(
		&mut self,
		_ctx: &mut <Self as Actor>::Context,
		sender: Addr<ConnectionInitiator>,
		connection: Addr<Connection>,
	) {
		use NetworkOutput::InfoRequested;
		println!("[NetworkManager] Got recieved info request");
		if let Some(delegate) = self.delegate.upgrade() {
			delegate.do_send(InfoRequested(connection));
		}
		self.remove_initiator(sender);
	}
}

impl Actor for NetworkManager {
	type Context = Context<Self>;

	fn started(&mut self, ctx: &mut Self::Context) {
		println!("[NetworkManager] started with config {:?}", self.config);
		let recipient = ctx.address().recipient();
		self.listener_addr.replace(NetworkListener::new(
			format!("0.0.0.0:{}", self.config.port),
			recipient,
		));
	}
}

impl Handler<NetworkMessage> for NetworkManager {
	type Result = ();
	fn handle(
		&mut self,
		msg: NetworkMessage,
		ctx: &mut <Self as actix::Actor>::Context,
	) -> <Self as Handler<NetworkMessage>>::Result {
		use NetworkMessage::{StartListening, StopListening};
		match msg {
			StartListening => self.start_listener(ctx),
			StopListening => self.stop_listener(ctx),
		}
	}
}

impl Handler<NetworkDataMessage> for NetworkManager {
	type Result = NetworkDataOutput;

	fn handle(
		&mut self,
		msg: NetworkDataMessage,
		ctx: &mut Self::Context,
	) -> Self::Result {
		match msg {
			NetworkDataMessage::IsListening => {
				NetworkDataOutput::IsListening(self.listener_addr.is_some())
			}
		}
	}
}

impl Handler<ListenerOutput> for NetworkManager {
	type Result = ();
	fn handle(
		&mut self,
		msg: ListenerOutput,
		ctx: &mut Self::Context,
	) -> Self::Result {
		use ListenerOutput::NewConnection;
		match msg {
			NewConnection(connection) => self.new_connection(ctx, connection),
		};
	}
}

impl Handler<InitiatorOutput> for NetworkManager {
	type Result = ();
	fn handle(
		&mut self,
		msg: InitiatorOutput,
		ctx: &mut Self::Context,
	) -> Self::Result {
		use InitiatorOutput::{ClientRequest, InfoRequest};
		match msg {
			ClientRequest(sender, addr, client_details) => {
				self.client_request(ctx, sender, addr, client_details)
			}
			InfoRequest(sender, addr) => self.info_request(ctx, sender, addr),
		}
	}
}

impl From<Builder> for NetworkManager {
	fn from(builder: Builder) -> Self {
		Self {
			config: Config {
				port: builder.port.unwrap_or_else(|| 5600),
			},
			listener_addr: None,
			delegate: builder.delegate,

			initiators: Vec::default(),
			config_manager: ConfigManager::shared().downgrade(),
		}
	}
}
