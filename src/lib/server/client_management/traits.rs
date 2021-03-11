use std::sync::Arc;

use uuid::Uuid;

/**
 * @michael-bailey
 */
pub trait TClientManager<TClient,TClientMessage> {
  fn addClient(&self, client: Arc<TClient>);
  fn removeClient(&self, id: Uuid);
  fn messageClient(&self, id: Uuid, msg: TClientMessage);
  fn tick(&self, );
}