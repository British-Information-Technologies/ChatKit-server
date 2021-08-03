use std::sync::Arc;

use async_trait::async_trait;

use serde::{Serialize,Deserialize};


#[async_trait]
trait Sender<'de, TMessage: Deserialize<'de> + Serialize>  {
	async fn send(self: &Arc<Self>, message: TMessage) -> Result<(), std::io::Error>;
	async fn recv(self: &Arc<Self>) -> Result<TMessage, std::io::Error>;
}
