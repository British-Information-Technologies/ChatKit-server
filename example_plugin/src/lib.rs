mod example;

use crate::example::ExamplePlugin;
use serverlib::plugin::plugin::Plugin;
use std::sync::Arc;

#[no_mangle]
pub extern "C" fn get_plugin() -> Plugin {
	Arc::new(ExamplePlugin::default())
}
