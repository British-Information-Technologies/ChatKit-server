use actix::{Actor, Addr};

use crate::config_manager::ConfigManager;

pub(super) struct Builder {
	pub(super) file_path: String,
}

impl Builder {
	pub(super) fn new() -> Self {
		Self {
			file_path: "./config_file.toml".to_owned(),
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

	pub(super) fn build(self) -> Addr<ConfigManager> {
		ConfigManager::from(self).start()
	}
}
