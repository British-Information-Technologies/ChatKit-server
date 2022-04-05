use std::sync::Arc;

/// # GetPluginFn
/// This defines the type for getting the plugin struct from a
pub type GetPluginFn = fn() -> Arc<dyn Plugin>;

/// # Plugin
/// This trait defines an interface for plugins to implement.
///
/// ## Methods
/// - details: This returns the details about the plugin.
/// - init: This defines the initialisation routine for the.
/// - run: defines a routine to be ran like a thread.
#[async_trait::async_trait]
pub trait Plugin {
	fn details(&self) -> PluginDetails;
	fn init(self: &Arc<Self>);
	async fn run(self: &Arc<Self>);
}
