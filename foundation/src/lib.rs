pub mod messages;
pub mod prelude;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
// use ring::signature::RsaPublicKeyComponents;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ClientDetails {
	pub uuid: Uuid,
	pub username: String,
	pub address: String,
	// pub public_key: Option<RsaPublicKeyComponents<>>,
}
