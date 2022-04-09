use futures::lock::Mutex;
use std::thread::sleep;
use std::time::Duration;

use serverlib::plugin::{plugin::Plugin, plugin_details::PluginDetails};
// use tokio::{sync::Mutex, time::sleep};
use serverlib::plugin::plugin::IPlugin;

#[derive(Debug)]
pub struct ExamplePlugin {
	number: Mutex<u8>,
}

impl Default for ExamplePlugin {
	fn default() -> Self {
		ExamplePlugin {
			number: Mutex::new(0),
		}
	}
}

#[async_trait::async_trait]
impl IPlugin for ExamplePlugin {
	fn details(&self) -> PluginDetails {
		PluginDetails {
			display_name: "ExamplePlugin",
			id: "io.github.michael-bailey.ExamplePlugin",
			version: "0.0.1",
			contacts: vec!["bailey-michael1@outlook.com"],
		}
	}

	fn init(&self) {
		println!("[ExamplePlugin]: example init")
	}

	async fn run(&self) {
		println!("Example!!!");
		sleep(Duration::new(1, 0));
		if let mut a = self.number.lock().await {
			*a += 1;
			println!("[ExamplePlugin]: example run {}", *a);
		}
	}

	fn deinit(&self) {
		todo!()
	}

	async fn event(&self) {
		todo!()
	}
}
