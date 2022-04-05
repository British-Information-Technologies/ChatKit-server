mod example;

use std::sync::Arc;
use serverlib::plugin::plugin::Plugin;
use crate::example::ExamplePlugin;

#[no_mangle]
extern fn get_plugin() -> Arc<dyn Plugin> {
	Arc::new(ExamplePlugin::default())
}