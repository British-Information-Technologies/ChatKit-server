pub mod messages;
pub mod prelude;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ClientDetails {
  pub uuid: Uuid,
  pub username: String,
  pub address: String,
}