use serde::{Deserialize, Serialize};
use tokio::{
	fs::File,
	io::{AsyncReadExt, AsyncSeekExt, AsyncWrite},
};

use std::io::{SeekFrom, ErrorKind};
use std::sync::Arc;
use crate::plugin::plugin::Plugin;

pub type PluginEntryObj = Arc<PluginEntry>;

#[derive(Serialize, Deserialize, Debug)]
pub enum PluginPermission {
	Read,
	Write,
	ReadWrite,
	None
}

/// # PluginEntry
/// a wrapper for plugins loaded into the server.
/// Used to provide an api for the plugin to use.
/// Also acts as gatekeeper to server data with permissions.
#[derive(Debug)]
pub struct PluginEntry {
	server_permission: PluginPermission,
	network_permission: PluginPermission,
	client_manager_permission: PluginPermission,
	client_permission: PluginPermission,

	plugin: Plugin
}


impl PluginEntry {
	pub fn new(plugin: Plugin) -> Arc<PluginEntry> {
		Arc::new(PluginEntry {
			server_permission: PluginPermission::None,
			network_permission: PluginPermission::None,
			client_manager_permission: PluginPermission::None,
			client_permission: PluginPermission::None,

			plugin
		})
	}

	pub fn start(&self) {
		let cont = self.plugin.clone();
		tokio::spawn(async move {
			println!("[PluginEntry:start] starting plugin: {:?}", cont.details().id);
			cont.init();
			loop {
				// Todo: Add code to stop loop once finished
				cont.run().await;
			}
			cont.deinit();
		});
	}
}



