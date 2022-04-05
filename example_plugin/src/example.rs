use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;
use serverlib::plugin::plugin::Plugin;
use serverlib::plugin::plugin_details::PluginDetails;

pub struct ExamplePlugin {
	number: Mutex<u8>
}

impl Default for ExamplePlugin {
	fn default() -> Self {
		ExamplePlugin {
			number: Mutex::new(0)
		}
	}
}

#[async_trait::async_trait]
impl Plugin for ExamplePlugin {
	fn details(&self) -> PluginDetails {
		PluginDetails {
			display_name: "ExamplePlugin",
			id: "io.github.michael-bailey.ExamplePlugin",
			version: "0.0.1",
			contacts: vec!["bailey-michael1@outlook.com"]
		}
	}

	fn init(self: &Arc<Self>) {
		println!("[ExamplePlugin]: example init")
	}

	async fn run(self: &Arc<Self>) {
		sleep(Duration::new(1,0)).await;
		if let mut a = self.number.lock().await {
			*a += 1;
			println!("[ExamplePlugin]: example run");
		}
	}
}