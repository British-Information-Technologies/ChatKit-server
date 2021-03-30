use crate::lib::server::client_management::client::ClientMessage;
use std::sync::Arc;

use uuid::Uuid;

/**
 * @michael-bailey
 */
pub trait TClientManager<TClient,TClientMessage> {
  fn add_client(&self, client: Arc<TClient>);
  fn remove_client(&self, uuid: Uuid);
  fn send_message_to_client(&self, uuid: Uuid, msg: ClientMessage);
}