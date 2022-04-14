use std::{collections::HashMap, io::Error, mem, sync::Arc};
use std::fs::Metadata;

use libloading::Library;
use tokio::fs::{create_dir, DirEntry, read_dir};
use tokio::sync::mpsc::Sender;
use tokio::sync::{Mutex, MutexGuard};
use serde::{Serialize, Deserialize};
use serde_json::StreamDeserializer;

use futures::future::join_all;
use futures::TryFutureExt;
use mlua::require_module_feature;

use crate::plugin::plugin::{GetPluginFn, Plugin};
use crate::plugin::plugin_entry::{PluginEntry, PluginEntryObj};

pub enum PluginManagerMessage {
	None,
}

/// # PluginManager
/// This struct handles the loading and unloading of plugins in the server
///
/// ## Attributes
/// - plugins: A [Vec] of all loaded plugins
/// - server_channel: A [Sender]
pub struct PluginManager<Out: 'static>
	where
		Out: From<PluginManagerMessage> + Send, {
	#[allow(dead_code)]
	plugins: Mutex<Vec<PluginEntryObj>>,

	#[allow(dead_code)]
	server_channel: Mutex<Sender<Out>>,
}

impl<Out: 'static> PluginManager<Out>
	where
		Out: From<PluginManagerMessage> + Send, {
	pub fn new(channel: Sender<Out>) -> Arc<Self> {
		Arc::new(Self {
			plugins: Mutex::new(Vec::new()),
			server_channel: Mutex::new(channel),
		})
	}

	pub async fn load(&self) -> Result<(), Error> {
		println!("[PluginManager]: loading plugins");
		println!(
			"[PluginManager]: from: {}",
			std::env::current_dir().unwrap().to_string_lossy()
		);

		if let Ok(mut plugins) = read_dir("./plugins").await {

			// Todo: - make this concurrent
			let mut plugin_vec = vec![];
			while let Some(next) = plugins.next_entry().await? {
				println!("{:?}", next);
				plugin_vec.push(next)
			}

			// get all entries by extension
			let entries: Vec<DirEntry> = plugin_vec.into_iter()
				.filter(|item| item.path().extension().unwrap_or_default() == "dylib")
				.collect();

			// get entry metadata
			let metadata: Vec<Metadata> = join_all(entries.iter()
				.map(|item| item.metadata())).await
				.into_iter()
				.filter_map(|item| item.ok())
				.collect();

			// convert correct ones to plugins
			let mut plugins: Vec<PluginEntryObj> = entries.into_iter().zip(metadata.into_iter())
				.filter(|(item, meta)| meta.is_file())
				.map(|item| item.0)
				.map(|item| unsafe {
					let lib = Library::new(item.path()).unwrap();
					let plugin_fn = lib.get::<GetPluginFn>("get_plugin".as_ref()).unwrap();
					PluginEntry::new(plugin_fn())
				})
				.collect();

			println!("[PluginManager:load] got plugins: {:?}", plugins);

			let mut self_vec = self.plugins.lock().await;
			let _ = mem::replace(&mut *self_vec, plugins);
		} else {
			create_dir("./plugins").await?;
		}

		self.plugins.lock().await
			.iter()
			.for_each(|item| item.start());

		Ok(())
	}
}
