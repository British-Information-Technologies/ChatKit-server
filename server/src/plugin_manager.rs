use std::collections::HashMap;
use std::sync::Arc;
use libloading::Library;
use crate::plugin::Plugin::Plugin;

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

	pub async fn load(&self) {
		unsafe {
			println!("[PluginManager]: loading plugins");
			println!("[PluginManager]: from: {}", std::env::current_dir().unwrap().to_string_lossy());
			let lib = Library::new("./plugins")?;
		}
	}
}