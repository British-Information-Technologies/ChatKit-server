use actix::{
	fut::wrap_future,
	Actor,
	ActorFutureExt,
	Addr,
	AsyncContext,
	Context,
	Handler,
	WeakAddr,
	WeakRecipient,
};
use foundation::ClientDetails;

use crate::{
	config_manager::{ConfigManager, ConfigManagerDataMessage, ConfigValue},
	network::{
		listener::{ListenerMessage, ListenerOutput, NetworkListener},
		network_manager::{
			messages::{NetworkMessage, NetworkOutput},
			Builder,
		},
		Connection,
		ConnectionInitiator,
		InitiatorOutput,
		NetworkDataMessage,
		NetworkDataOutput,
	},
};

/// # NetworkManager
/// this struct will handle all networking functionality.
///
pub struct NetworkManager {
	config_manager: WeakAddr<ConfigManager>,
	listener_addr: Option<Addr<NetworkListener>>,
	delegate: WeakRecipient<NetworkOutput>,
	initiators: Vec<Addr<ConnectionInitiator>>,
}

impl NetworkManager {
	pub fn new(delegate: WeakRecipient<NetworkOutput>) -> Addr<NetworkManager> {
		NetworkManager {
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

		println!("[NetworkManager] got Listen message");

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
	fn remove_initiator(&mut self, sender: WeakAddr<ConnectionInitiator>) {
		if let Some(sender) = sender.upgrade() {
			let index = self.initiators.iter().position(|i| *i == sender).unwrap();
			println!("[NetworkManager] removed initiator at:{}", index);
			let _ = self.initiators.remove(index);
		}
	}

	/// handles a initiator client request
	/// this will, forward the conenction and client details
	/// to the server actor to be dispatched to the appropriate
	/// manager
	#[inline]
	fn client_request(
		&mut self,
		_ctx: &mut <Self as Actor>::Context,
		sender: WeakAddr<ConnectionInitiator>,
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
		sender: WeakAddr<ConnectionInitiator>,
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
		println!("[NetworkManager] Starting");
		let config_mgr = self.config_manager.clone().upgrade();

		if let Some(config_mgr) = config_mgr {
			let fut = wrap_future(config_mgr.send(
				ConfigManagerDataMessage::GetValue("Network.Port".to_owned()),
			))
			.map(
				|out, actor: &mut NetworkManager, ctx: &mut Context<NetworkManager>| {
					use crate::config_manager::ConfigManagerDataResponse::GotValue;

					println!("[NetworkManager] got config manager value {:?}", out);

					let recipient = ctx.address().recipient();

					let port = if let Ok(GotValue(Some(ConfigValue::Number(port)))) = out
					{
						port
					} else {
						5600
					};
					println!("[NetworkManager] got port: {:?}", port);
					let nl = NetworkListener::new(
						format!("0.0.0.0:{}", port),
						recipient.downgrade(),
					);
					nl.do_send(ListenerMessage::StartListening);
					actor.listener_addr.replace(nl);
				},
			);
			ctx.spawn(fut);
		}
	}

	fn stopped(&mut self, ctx: &mut Self::Context) {
		println!("[NetworkManager] network manager stopped");
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
		_ctx: &mut Self::Context,
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
			NewConnection(connection) => {
				println!("new connection");
				self.new_connection(ctx, connection)
			}
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
			listener_addr: None,
			delegate: builder.delegate,

			initiators: Vec::default(),
			config_manager: ConfigManager::shared().downgrade(),
		}
	}
}
