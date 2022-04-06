use crate::plugin::plugin::{GetPluginFn, Plugin};

use libloading::Library;

use std::collections::HashMap;
use std::io::Error;
use std::sync::Arc;
use tokio::fs::{create_dir, read_dir};

/// # PluginManager
/// This struct handles the loading and unloading of plugins in the server
///
/// ## Attributes
/// - plugins: A [HashMap] of all loaded plugins
pub struct PluginManager {
	#[allow(dead_code)]
	plugins: HashMap<String, Arc<dyn Plugin>>,
}

impl PluginManager {
	pub fn new() -> Arc<Self> {
		Arc::new(Self {
			plugins: HashMap::new(),
		})
	}

	pub async fn load(&self) -> Result<(), Error> {
		println!("[PluginManager]: loading plugins");
		println!(
			"[PluginManager]: from: {}",
			std::env::current_dir().unwrap().to_string_lossy()
		);

		if let Ok(mut plugins) = read_dir("./plugins").await {
			while let Some(child) = plugins.next_entry().await? {
				let metadata = child.metadata().await?;
				if metadata.is_file() && child.path().extension().unwrap() == "dylib" {
					println!(
						"[PluginManager]: Library at:{}",
						child.path().to_string_lossy()
					);
					unsafe {
						let lib = Library::new(child.path()).unwrap();
						let plugin_fn = lib.get::<GetPluginFn>("get_plugin".as_ref()).unwrap();
						let plugin: Arc<dyn Plugin> = plugin_fn();

						plugin.init();

						let cont = plugin.clone();

						tokio::spawn(async move {
							loop {
								cont.run().await;
							}
						});

						println!("[PluginManager]: got details: {}", plugin.details());
					};
				}
			}
		} else {
			create_dir("./plugins").await?;
		}
		Ok(())
	}
}
