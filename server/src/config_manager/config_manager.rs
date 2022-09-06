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
		get_args,
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
	file: File,
	stored: ConfigValue,
	root: ConfigValue,
	subscribers: Vec<Recipient<ObservableMessage<ConfigManagerOutput>>>,
}

impl ConfigManager {
	pub fn shared(file: Option<String>) -> Addr<Self> {
		INIT.call_once(|| {
			// Since this access is inside a call_once, before any other accesses, it is safe
			unsafe {
				let mut file = Self::get_file();
				let shared = Self::new(file);

				SHARED = Some(shared);
			}
		});
		unsafe { SHARED.clone().unwrap() }
	}

	fn new(mut file: File) -> Addr<Self> {
		let mut output = String::new();
		file.read_to_string(&mut output)
			.expect("failed to read from file");

		let stored = output
			.parse::<Value>()
			.map(|v| v.into())
			.ok()
			.unwrap_or_else(|| ConfigValue::Dict(BTreeMap::new()));

		let root = stored.clone();

		Self {
			file,
			root,
			stored,
			subscribers: Vec::default(),
		}
		.start()
	}

	fn get_file() -> File {
		let default = "./config_file.toml".to_owned();

		let args = get_args();
		let file_path =
			args.get_one::<String>("config file").unwrap_or(&default);

		OpenOptions::new()
			.write(true)
			.read(true)
			.open(file_path)
			.ok()
			.unwrap()
	}
}

impl ConfigManager {
	pub fn get_value(&self, key: String) -> Result<ConfigValue, &'static str> {
		use ConfigValue::Dict;

		if let Dict(dict) = &self.root {
			let opt_value = dict.get(&key);
			return if let Some(value) = opt_value {
				Ok(value.clone())
			} else {
				Err("[ConfigManager] get_value: Value does not exist")
			};
		}
		Err("[ConfigManager] get_value: Key does not exist")
	}

	// this doesn't work for now
	pub fn set_value(
		&mut self,
		key: String,
		value: ConfigValue,
	) -> Result<ConfigManagerDataResponse, &'static str> {
		use ConfigManagerDataResponse::SetValue;
		use ConfigValue::Dict;

		if let Dict(dict) = &mut self.root {
			dict.insert(key, value);
			Ok(SetValue)
		} else {
			Err("[ConfigManager] set_value: What the hell did ou do wrong")
		}
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
			ConfigManagerDataMessage::SetValue(key, value) => {
				self.set_value(key, value)
			}
			ConfigManagerDataMessage::SoftSetValue(_, _) => Ok(SetValue),
		}
	}
}
