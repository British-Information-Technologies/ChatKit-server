use std::sync::Arc;
use async_trait::async_trait;


/// This is used with all managers to implement multitasking
#[async_trait]
pub trait IManager {
	/// this is used to get a future that can be awaited
	async fn run(self: Arc<Self>);
	
	/// This is used to start a future through tokio
	async fn start(self: &Arc<Self>);
}