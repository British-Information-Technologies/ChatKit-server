pub mod client;
pub mod messages;
pub mod models;
pub mod networking;
pub mod prelude;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/**
 * # ClientDetails.
 * This defines the fileds a client would want to send when connecitng
 * uuid:          the unique id of the user.
 * username:      the users user name.
 * address:       the ip address of the connected user.
 * public_key:    the public key used when sending messages to the user.
 */
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct ClientDetails {
	pub uuid: Uuid,
	pub username: String,
	pub address: String,
	pub public_key: Option<Vec<u8>>,
}
