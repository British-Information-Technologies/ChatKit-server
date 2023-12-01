//! Contains all the structures for managing chat storage.
//! it contains:
//! - ChatManager
//! - Messages
//! - Mesage type

mod actor;

mod messages;

pub(crate) use actor::ChatManager;
pub(crate) use messages::{
	ChatManagerDataMessage,
	ChatManagerDataResponse,
	ChatManagerMessage,
};
