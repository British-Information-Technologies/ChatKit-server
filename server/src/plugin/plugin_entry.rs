use crate::plugin::plugin_interface::IPluginInterface;
use crate::plugin::PluginInterface;
use foundation::event::Event;

use crate::event_type::EventType;

use foundation::event::EventResult;
use foundation::event::IResponder;
use serde::{Deserialize, Serialize};
use std::sync::Weak;

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
pub(crate) struct PluginEntry<T>
where
	T: Sync + Send,
{
	server_permission: PluginPermission,
	network_permission: PluginPermission,
	client_manager_permission: PluginPermission,
	client_permission: PluginPermission,

	state: Arc<Mutex<PluginExecutionState>>,

	plugin: Plugin<EventType<'static>>,
}

impl<T> PluginEntry<T>
where
	T: Sync + Send,
{
	pub fn new(plugin: Plugin<EventType>) -> Arc<PluginEntry<T>> {
		let entry = Arc::new(PluginEntry {
			server_permission: PluginPermission::None,
			network_permission: PluginPermission::None,
			client_manager_permission: PluginPermission::None,
			client_permission: PluginPermission::None,

			state: Arc::new(Mutex::new(Stopped)),

			plugin: plugin.clone(),
		});

		let entry_ref = entry.clone() as PluginInterface<T>;

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

impl<T> IPluginInterface<T> for PluginEntry {
	fn send_event(&self, _event: Event<EventType>) -> Receiver<EventResult> {
		todo!()
	}
}

impl IResponder<EventType<'_>> for PluginEntry {
	fn on_event(&self, event: Event<EventType>) {
		use EventType::{ClientAdded, Custom, NewConnection};
		use PluginPermission::{None, Read, ReadWrite, Write};

		match (
			&event.r#type,
			&self.network_permission,
			&self.client_manager_permission,
			&self.client_permission,
			&self.server_permission,
		) {
			(NewConnection, Read | ReadWrite, _, _, _) => self.plugin.on_event(event),
			(ClientAdded(id), _, Read | ReadWrite, _, _) => self.plugin.on_event(event),
			(Custom("ping"), _, _, _, _) => println!("[PluginEntry:on_event] Ping!"),
			_ => println!("[PluginEntry:on_event] not handled"),
		};
	}
	fn get_next(&self) -> Option<Weak<dyn IResponder<EventType>>> {
		todo!()
	}
}
