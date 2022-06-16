use std::{
	sync::{Arc, Weak},
	time::Duration,
};

use async_trait::async_trait;
use tokio::time::sleep;

/// # IManager
/// This is used with all managers to implement multitasking
///
/// ## Methods
/// - init: gets executed once before a tokio task is created
/// - run: gets called once every tick in a tokio task
/// - start: runs the init function then creates the tokio task for the run function
#[async_trait]
pub trait IManager {
	/// This defines some setup before the tokio loop is started
	async fn init(self: &Arc<Self>)
	where
		Self: Send + Sync + 'static,
	{
	}

	/// this is used to get a future that can be awaited
	async fn run(self: &Arc<Self>);

	/// This is used to start a future through tokio
	fn start(self: &Arc<Self>)
	where
		Self: Send + Sync + 'static,
	{
		let weak_self: Weak<Self> = Arc::downgrade(self);

		// this looks horrid but works
		tokio::spawn(async move {
			let weak_self = weak_self.clone();

			let a = weak_self.upgrade().unwrap();
			a.init().await;
			drop(a);

			loop {
				sleep(Duration::new(1, 0)).await;
				if let Some(manager) = Weak::upgrade(&weak_self) {
					manager.run().await
				}
				()
			}
		});
	}
}

trait Visitor<T: IManager> {
	fn visit(&self, message: T);
}
