use std::collections::HashMap;
use std::io::Error;
use std::sync::Arc;
use libloading::Library;
use tokio::fs::{create_dir, File, read_dir};
use crate::plugin::plugin::{Plugin, PluginDetailsFn, TestPluginFn};
use crate::plugin::plugin_details::PluginDetails;

/// # PluginManager
///
/// This struct handles the loading and unloading of plugins in the server
///
/// ## Attributes
/// - plugins: a hash_map of all loaded plugins
pub struct PluginManager {
	plugins: HashMap<String, Arc<dyn Plugin>>
}

impl PluginManager {
	pub fn new() -> Arc<Self>{
		return Arc::new(Self {
			plugins: HashMap::new()
		})
	}

	pub async fn load(&self) -> Result<(), Error> {

		println!("[PluginManager]: loading plugins");
		println!("[PluginManager]: from: {}", std::env::current_dir().unwrap().to_string_lossy());

		if let Ok( mut plugins) = read_dir("./plugins").await {
			while let Some(child) = plugins.next_entry().await? {
				let metadata = child.metadata().await?;

				if metadata.is_file() && child.path().extension().unwrap() == "dylib" {
					println!("[PluginManager]: Library at:{}", child.path().to_string_lossy());
					unsafe {
						let lib = Library::new(child.path()).unwrap();
						let get_details = lib.get::<PluginDetailsFn>("details".as_ref()).unwrap();
						let details = get_details();
						println!("[PluginManager]: got details: {}", details);
					};
				}
			}
		} else {
			create_dir("./plugins").await?;
		}
		Ok(())
	}
}