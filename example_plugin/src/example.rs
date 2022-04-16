use futures::lock::Mutex;
use std::thread::sleep;
use std::time::Duration;

use serverlib::plugin::{Plugin, PluginDetails};
// use tokio::{sync::Mutex, time::sleep};
use serverlib::plugin::IPlugin;

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
		let mut a = self.number.lock().await;
		*a = a.overflowing_add(1).0;
		println!("[ExamplePlugin]: example run {}", *a);
	}

	fn deinit(&self) {
		if let Some(mut lock) = self.number.try_lock() {
			*lock = 0;
		}
	}

	async fn event(&self) {
		println!("Not Implemented");
	}
}
