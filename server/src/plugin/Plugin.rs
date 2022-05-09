use crate::plugin::WeakPluginInterface;
use foundation::event::Event;
use std::fmt::Debug;
use std::sync::Arc;

use crate::plugin::plugin_details::PluginDetails;
use std::sync::Arc;

/// # Plugin
/// Type alias for plugin objects.
pub type Plugin<T> = Arc<dyn IPlugin<T>>;

/// # GetPluginFn
/// This defines the type for getting the plugin struct from a
pub type GetPluginFn<T> = fn() -> Plugin<T>;

/// # Plugin
/// This trait defines an interface for plugins to implement.
///
/// ## Methods
/// - details: This returns the details about the plugin.
/// - init: Defines the initialisation routine for the plugin.
/// - run: defines a routine to be ran like a thread by the plugin manager.
/// - deinit: Defines the deinitalisation routine for the plugin
#[async_trait::async_trait]
pub trait IPlugin<T>: Send + Sync + Debug {
	fn details(&self) -> PluginDetails;
	fn on_event(&self, event: Event<T>);

	fn set_interface(&self, interface: WeakPluginInterface<T>);

	fn init(&self);
	async fn run(&self);
	fn deinit(&self);
}
