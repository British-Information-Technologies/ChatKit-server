use std::sync::Arc;

use async_trait::async_trait;

use serde::de::DeserializeOwned;
use serde::Serialize;

pub type TransformerFn = Box<dyn Fn(&[u8]) -> Vec<u8> + Send + Sync>;

#[async_trait]
pub trait StreamMessageSender {
	async fn send<TOutMessage: Serialize + Send>(
		self: &Arc<Self>,
		message: TOutMessage,
	) -> Result<(), std::io::Error>;
	async fn recv<'de, TInMessage: DeserializeOwned + Send>(
		self: &Arc<Self>,
	) -> Result<TInMessage, std::io::Error>;
}