use crate::event::Event;
use crate::event::EventResult;
use crate::plugin::plugin_interface::IPluginInterface;
use crate::plugin::PluginInterface;
use serde::{Deserialize, Serialize};

use futures::channel::oneshot::Receiver;

use crate::plugin::plugin::Plugin;
use crate::plugin::plugin_entry::PluginExecutionState::{Paused, Running, Stopped};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;

pub(crate) type PluginEntryObj = Arc<PluginEntry>;

#[derive(Serialize, Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum PluginPermission {
	Read,
	Write,
	ReadWrite,
	None,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) enum PluginExecutionState {
	Running,
	Paused,
	Stopped,
}

/// # PluginEntry
/// a wrapper for plugins loaded into the server.
/// Used to provide an api for the plugin to use.
/// Also acts as gatekeeper to server data with permissions.
#[derive(Debug)]
pub(crate) struct PluginEntry {
	server_permission: PluginPermission,
	network_permission: PluginPermission,
	client_manager_permission: PluginPermission,
	client_permission: PluginPermission,

	state: Arc<Mutex<PluginExecutionState>>,

	plugin: Plugin,
}

impl PluginEntry {
	pub fn new(plugin: Plugin) -> Arc<PluginEntry> {
		let entry = Arc::new(PluginEntry {
			server_permission: PluginPermission::None,
			network_permission: PluginPermission::None,
			client_manager_permission: PluginPermission::None,
			client_permission: PluginPermission::None,

			state: Arc::new(Mutex::new(Stopped)),

			plugin: plugin.clone(),
		});

		let entry_ref = entry.clone() as PluginInterface;

		plugin.set_interface(Arc::downgrade(&entry_ref));
		entry
	}

	pub(crate) async fn getState(&self) -> PluginExecutionState {
		*self.state.lock().await
	}

	pub fn start(&self) {
		let cont = self.plugin.clone();
		let state = self.state.clone();
		tokio::spawn(async move {
			let local_state = state.clone();
			let mut lock = local_state.lock().await;
			match *lock {
				Running => (),
				Paused => {
					*lock = Running;
				}
				Stopped => {
					tokio::spawn(async move {
						cont.init();
						let mut lock = state.lock().await;
						*lock = Running;
						loop {
							match *lock {
								Running => cont.run().await,
								Paused => sleep(Duration::new(1, 0)).await,
								Stopped => break,
							}
						}
						cont.deinit()
					});
				}
			}
		});
	}

	pub fn pause(&self) {
		let state = self.state.clone();
		tokio::spawn(async move {
			let mut lock = state.lock().await;
			match *lock {
				Running => {
					*lock = Paused;
				}
				Paused => (),
				Stopped => (),
			}
		});
	}

	pub fn stop(&self) {
		let state = self.state.clone();
		tokio::spawn(async move {
			let mut lock = state.lock().await;
			match *lock {
				Running => {
					*lock = Stopped;
				}
				Paused => {
					*lock = Stopped;
				}
				Stopped => (),
			}
		});
	}
}

impl IPluginInterface for PluginEntry {
	fn send_event(&self, _event: Event) -> Receiver<EventResult> {
		todo!()
	}
}
