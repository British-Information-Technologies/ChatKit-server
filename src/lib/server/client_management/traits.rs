use std::sync::Arc;

use uuid::Uuid;

/**
 * @michael-bailey
 */
pub trait TClientManager<TClient,TClientMessage> {
  fn add_client(&self, client: Arc<TClient>);
  fn remove_client(&self, id: Uuid);
  fn message_client(&self, id: Uuid, msg: TClientMessage);
  fn tick(&self, );
}