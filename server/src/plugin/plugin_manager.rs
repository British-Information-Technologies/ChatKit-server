use std::fs::Metadata;
use std::{io::Error, mem, sync::Arc};

use libloading::Library;

use tokio::fs::{create_dir, read_dir, DirEntry};
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;

use futures::future::join_all;

use crate::plugin::plugin::GetPluginFn;
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
	Out: From<PluginManagerMessage> + Send,
{
	#[allow(dead_code)]
	plugins: Mutex<Vec<PluginEntryObj>>,

	#[allow(dead_code)]
	server_channel: Mutex<Sender<Out>>,
}

impl<Out: 'static> PluginManager<Out>
where
	Out: From<PluginManagerMessage> + Send,
{
	/// Creates a new plugin manager with sender.
	pub fn new(channel: Sender<Out>) -> Arc<Self> {
		Arc::new(Self {
			plugins: Mutex::new(Vec::new()),
			server_channel: Mutex::new(channel),
		})
	}

	/// Starts loading plugins from the plugins directory.
	/// If this directory isn't found then create it get created.
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
			let entries: Vec<DirEntry> = plugin_vec
				.into_iter()
				.filter(|item| item.path().extension().unwrap_or_default() == "dylib")
				.collect();

			// get entry metadata
			#[allow(clippy::needless_collect)] // This is a false positive. Collect is needed here
			let metadata: Vec<Metadata> = join_all(entries.iter().map(|item| item.metadata()))
				.await
				.into_iter()
				.filter_map(|item| item.ok())
				.collect();

			// convert correct ones to plugins
			let plugins: Vec<PluginEntryObj> = entries
				.into_iter()
				.zip(metadata.into_iter())
				.filter(|(_item, meta)| meta.is_file())
				.map(|item| item.0)
				.map(|item| unsafe {
					let lib = Library::new(item.path()).unwrap();
					let plugin_fn = lib.get::<GetPluginFn<()>>("get_plugin".as_ref()).unwrap();
					PluginEntry::new(plugin_fn())
				})
				.collect();

			println!("[PluginManager:load] got plugins: {:?}", plugins);

			let mut self_vec = self.plugins.lock().await;
			let _ = mem::replace(&mut *self_vec, plugins);
		} else {
			create_dir("./plugins").await?;
		}

		self
			.plugins
			.lock()
			.await
			.iter()
			.for_each(|item| item.start());

		Ok(())
	}
}
