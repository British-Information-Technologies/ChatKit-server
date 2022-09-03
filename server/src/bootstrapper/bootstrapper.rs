use crate::bootstrapper::builder::Builder;
use crate::config_manager::ConfigManagerDataMessage::SoftSetValue;
use crate::config_manager::{ConfigManager, ConfigValue};
use crate::Server;
use actix::fut::wrap_future;
use actix::{
	Actor, ActorFutureExt, ActorStreamExt, ActorTryFutureExt, Addr,
	AsyncContext, Context,
};
use clap::ArgMatches;
use std::fs::OpenOptions;
use tokio::fs::File;

/// # Bootstrapper
/// This class acts as the init daemon of the server.
/// it creates the necessary actors required by the server before it inits.
/// it handles things like fetching configurations and uses them in server creation.
/// It takes the args passed into the program and overrides the corresponding fields in the config manager
pub struct Bootstrapper {
	args: ArgMatches,
	server: Option<Addr<Server>>,
}

impl Bootstrapper {
	pub fn create() -> Builder {
		Builder::new()
	}
}

impl Actor for Bootstrapper {
	type Context = Context<Self>;

	fn started(&mut self, ctx: &mut Self::Context) {
		let config_file = self
			.args
			.get_one::<String>("config file")
			.map(|v| v.clone());
		let port = self.args.get_one::<u16>("port").map(|v| *v);
		let name = self
			.args
			.get_one::<String>("server name")
			.map(|v| v.clone());
		let owner = self
			.args
			.get_one::<String>("server owner")
			.map(|v| v.clone());

		let fut = wrap_future(async move {
			let config_manager = ConfigManager::shared(config_file);

			if let Some(port) = port {
				config_manager
					.send(SoftSetValue(
						"server.port".into(),
						ConfigValue::Number(port.into()),
					))
					.await;
			}

			if let Some(name) = name {
				let _ = config_manager
					.send(SoftSetValue(
						"server.name".into(),
						ConfigValue::String(name),
					))
					.await;
			}

			if let Some(owner) = owner {
				let _ = config_manager
					.send(SoftSetValue(
						"server.owner".into(),
						ConfigValue::String(owner),
					))
					.await;
			}

			config_manager
		})
		.map(|val, obj: &mut Bootstrapper, _ctx| {
			obj.server = Server::create(val).build().into();
		});

		ctx.spawn(fut);
	}
}

impl From<Builder> for Bootstrapper {
	fn from(b: Builder) -> Self {
		Self {
			args: b.args,
			server: None,
		}
	}
}
