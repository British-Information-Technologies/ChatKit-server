use actix::{Actor, Addr};

use crate::config_manager::{arg_parser::Arguments, ConfigManager};

pub(super) struct Builder {
	pub(super) file_path: String,
	pub(super) args: Option<Arguments>,
}

impl Builder {
	pub(super) fn new() -> Self {
		Self {
			file_path: "./config_file.toml".to_owned(),
			args: None,
		}
	}

	#[allow(dead_code)]
	pub fn config_path(mut self, path: impl Into<String>) -> Self {
		self.file_path = path.into();
		self
	}

	pub fn set_config_path(&mut self, path: impl Into<String>) {
		self.file_path = path.into();
	}

	pub fn args(mut self, args: Arguments) -> Self {
		self.args.replace(args);
		self
	}

	pub fn set_args(&mut self, args: Arguments) {
		self.args.replace(args);
	}

	pub(super) fn build(self) -> Addr<ConfigManager> {
		ConfigManager::from(self).start()
	}
}
