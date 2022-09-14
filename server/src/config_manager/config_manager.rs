use std::{
	collections::BTreeMap,
	fs::{File, OpenOptions},
	io::Read,
	sync::Once,
};

use actix::{Actor, Addr, Context, Handler, Recipient};
use clap::Parser;
use toml::Value;

use crate::{
	config_manager::{
		arg_parser::Arguments,
		builder::Builder,
		messages::{
			ConfigManagerDataMessage, ConfigManagerDataResponse,
			ConfigManagerOutput,
		},
		types::ConfigValue::{Dict, Number, String as ConfigString},
		ConfigValue,
	},
	prelude::messages::ObservableMessage,
};

static mut SHARED: Option<Addr<ConfigManager>> = None;
static INIT: Once = Once::new();

#[allow(dead_code)]
pub(crate) struct ConfigManager {
	file: File,
	stored: ConfigValue,
	root: ConfigValue,
	subscribers: Vec<Recipient<ObservableMessage<ConfigManagerOutput>>>,
}

// static methods
impl ConfigManager {
	pub fn shared() -> Addr<Self> {
		INIT.call_once(|| {
			let builder = Self::create().args(Arguments::parse()).build();
			unsafe { SHARED = Some(builder) }
		});
		unsafe { SHARED.clone().unwrap() }
	}

	pub(super) fn create() -> Builder {
		Builder::new()
	}
}

// instance methods
impl ConfigManager {
	pub fn get_value(&self, key: String) -> Option<ConfigValue> {
		if let Dict(dict) = &self.root {
			dict.get(&key).cloned()
		} else {
			None
		}
	}

	pub fn set_value(
		&mut self,
		key: String,
		value: Option<ConfigValue>,
	) -> Option<ConfigValue> {
		value.and_then(|value| {
			if let (Dict(stored), Dict(root)) =
				(&mut self.stored, &mut self.root)
			{
				stored.insert(key.clone(), value.clone());
				root.insert(key.clone(), value.clone());
				Some(value)
			} else {
				None
			}
		})
	}

	// this doesn't work for now
	pub fn soft_set_value(
		&mut self,
		key: String,
		value: Option<ConfigValue>,
	) -> Option<ConfigValue> {
		value.and_then(|value| {
			if let Dict(root) = &mut self.root {
				root.insert(key, value.clone());
				Some(value)
			} else {
				None
			}
		})
	}
}

impl Actor for ConfigManager {
	type Context = Context<Self>;

	fn started(&mut self, _ctx: &mut Self::Context) {
		println!("[ConfigManager] starting");
		println!("[ConfigManager] started");
	}
}

impl Handler<ConfigManagerDataMessage> for ConfigManager {
	type Result = ConfigManagerDataResponse;

	fn handle(
		&mut self,
		msg: ConfigManagerDataMessage,
		_ctx: &mut Self::Context,
	) -> Self::Result {
		use ConfigManagerDataResponse::{GotValue, SetValue, SoftSetValue};

		match msg {
			ConfigManagerDataMessage::GetValue(val) => {
				GotValue(self.get_value(val))
			}
			ConfigManagerDataMessage::SetValue(key, value) => {
				SetValue(key.clone(), self.set_value(key, value))
			}
			ConfigManagerDataMessage::SoftSetValue(key, value) => {
				SoftSetValue(key.clone(), self.soft_set_value(key, value))
			}
		}
	}
}

impl From<Builder> for ConfigManager {
	fn from(builder: Builder) -> Self {
		println!("got args: {:#?}", builder.args);

		let mut file = OpenOptions::new()
			.write(true)
			.read(true)
			.create(true)
			.open(builder.file_path)
			.ok()
			.unwrap();

		let mut output = String::new();
		file.read_to_string(&mut output)
			.expect("failed to read from file");

		let stored = output
			.parse::<Value>()
			.map(|v| v.into())
			.ok()
			.unwrap_or_else(|| Dict(BTreeMap::new()));

		let mut root = stored.clone();
		if let Dict(root) = &mut root {
			builder.args.map(|v| {
				v.port.map(|p| {
					root.insert("Network.Port".to_owned(), Number(p.into()))
				});

				v.name.map(|n| {
					root.insert(
						"Server.Name".to_owned(),
						ConfigString(n.into()),
					)
				});

				v.owner.map(|o| {
					root.insert(
						"Server.Owner".to_owned(),
						ConfigString(o.into()),
					)
				});
			});
		}

		Self {
			file,
			root,
			stored,
			subscribers: Vec::default(),
		}
	}
}
