use crate::plugin::WeakPluginInterface;
use std::fmt::Debug;
use std::sync::Arc;

use crate::plugin::plugin_details::PluginDetails;
use std::sync::Arc;

/// # Plugin
/// Type alias for plugin objects.
pub type Plugin = Arc<dyn IPlugin>;

/// # GetPluginFn
/// This defines the type for getting the plugin struct from a
pub type GetPluginFn = fn() -> Plugin;

/// # Plugin
/// This trait defines an interface for plugins to implement.
///
/// ## Methods
/// - details: This returns the details about the plugin.
/// - init: Defines the initialisation routine for the plugin.
/// - run: defines a routine to be ran like a thread by the plugin manager.
/// - deinit: Defines the deinitalisation routine for the plugin
#[async_trait::async_trait]
pub trait IPlugin: Send + Sync + Debug {
	fn details(&self) -> PluginDetails;
	async fn event(&self);

	fn set_interface(&self, interface: WeakPluginInterface);

	fn init(&self);
	async fn run(&self);
	fn deinit(&self);
}
