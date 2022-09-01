use std::{
	collections::BTreeMap,
	fs::{File, OpenOptions},
	io::Read,
	sync::Once,
};

use actix::{Actor, Addr, Context, Handler, Recipient};
use toml::Value;

use crate::{
	config_manager::{
		messages::{
			ConfigManagerDataMessage, ConfigManagerDataResponse,
			ConfigManagerOutput,
		},
		types::ConfigValue,
	},
	prelude::messages::ObservableMessage,
};

static mut SHARED: Option<Addr<ConfigManager>> = None;
static INIT: Once = Once::new();

pub(crate) struct ConfigManager {
	_file: Option<File>,
	_stored: ConfigValue,
	root: ConfigValue,
	_subscribers: Vec<Recipient<ObservableMessage<ConfigManagerOutput>>>,
}

impl ConfigManager {
	pub fn new(file: Option<String>) -> Addr<Self> {
		INIT.call_once(|| {
			// Since this access is inside a call_once, before any other accesses, it is safe
			unsafe {
				// todo: add proper error handling
				let file = OpenOptions::new()
					.write(true)
					.read(true)
					.open(file.unwrap_or("./config_file.toml".into()))
					.ok();

				let mut output = String::new();
				file.as_ref().map(|mut v| {
					v.read_to_string(&mut output)
						.expect("failed to read from file")
				});

				let stored = output
					.parse::<Value>()
					.map(|v| v.into())
					.ok()
					.unwrap_or_else(|| ConfigValue::Dict(BTreeMap::new()));

				let root = stored.clone();

				let shared = Self {
					_file: file,
					root,
					_stored: stored,
					_subscribers: Vec::default(),
				}
				.start();
				SHARED = Some(shared);
			}
		});
		unsafe { SHARED.clone().unwrap() }
	}
}

impl ConfigManager {
	pub(crate) fn get_value(
		&self,
		val_path: String,
	) -> Result<ConfigValue, &'static str> {
		use ConfigValue::{Array, Dict};

		let path: Vec<String> = val_path.split('.').map(|v| v.into()).collect();
		let mut current_node: &ConfigValue = &self.root;

		for i in path {
			match current_node {
				Dict(v) => match v.get(&i) {
					Some(v) => current_node = v,
					None => return Err("path does not exist"),
				},
				Array(v) => {
					if let Ok(index) = i.parse::<usize>() {
						current_node = &v[index];
					}
				}
				_ => return Err("invalid path"),
			}
		}
		Ok(current_node.clone())
	}
}

impl Actor for ConfigManager {
	type Context = Context<Self>;

	fn started(&mut self, _ctx: &mut Self::Context) {}
}

impl Handler<ObservableMessage<ConfigManagerOutput>> for ConfigManager {
	type Result = ();

	fn handle(
		&mut self,
		_msg: ObservableMessage<ConfigManagerOutput>,
		_ctx: &mut Self::Context,
	) -> Self::Result {
		todo!()
	}
}

impl Handler<ConfigManagerDataMessage> for ConfigManager {
	type Result = Result<ConfigManagerDataResponse, &'static str>;

	fn handle(
		&mut self,
		msg: ConfigManagerDataMessage,
		_ctx: &mut Self::Context,
	) -> Self::Result {
		use ConfigManagerDataResponse::{GotValue, SetValue};

		match msg {
			ConfigManagerDataMessage::GetValue(val) => {
				Ok(GotValue(self.get_value(val)?))
			}
			ConfigManagerDataMessage::SetValue(_, _) => Ok(SetValue),
			ConfigManagerDataMessage::SoftSetValue(_, _) => Ok(SetValue),
		}
	}
}
