use std::sync::{Arc, Weak};
use std::time::Duration;
use async_trait::async_trait;
use tokio::time::sleep;


/// This is used with all managers to implement multitasking
#[async_trait]
pub trait IManager {

	/// This defines some setup before the tokio loop is started
	async fn init(self: &Arc<Self>)
		where
			Self: Send + Sync + 'static
	{}

	/// this is used to get a future that can be awaited
	async fn run(self: &Arc<Self>);

	/// This is used to start a future through tokio
	fn start(self: &Arc<Self>)
		where
			Self: Send + Sync + 'static
	{
		let weak_self: Weak<Self> = Arc::downgrade(self);

		// this looks horrid but works
		tokio::spawn(async move {

			let weak_self = weak_self.clone();

			let a  = weak_self.upgrade().unwrap();
			a.init().await;
			drop(a);

			loop {
				sleep(Duration::new(1,0)).await;
				if let Some(manager) = Weak::upgrade(&weak_self) {
					manager.run().await
				} else { () }
			}
		});
	}
}