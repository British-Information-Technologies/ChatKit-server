//! Contains all the structures for managing chat storage.
//! it contains:
//! - ChatManager
//! - Messages
//! - Mesage type

mod actor;
mod message_type;
mod messages;

pub(crate) use actor::ChatManager;
use message_type::Message;
pub(crate) use messages::{
	ChatManagerDataMessage,
	ChatManagerDataResponse,
	ChatManagerMessage,
};
