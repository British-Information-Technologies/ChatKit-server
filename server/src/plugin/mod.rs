mod plugin;
mod plugin_details;
mod plugin_entry;
mod plugin_interface;
mod plugin_manager;
mod plugin_permissions;

pub use plugin::{IPlugin, Plugin};
pub use plugin_details::PluginDetails;
pub(crate) use plugin_manager::{PluginManager, PluginManagerMessage};
