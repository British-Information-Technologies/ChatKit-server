//! Contains all the structures for managing chat storage.
//! it contains:
//! - ChatManager
//! - Messages
//! - Mesage type

mod chat_manager;
mod message_type;
mod messages;

pub(crate) use chat_manager::ChatManager;
use message_type::Message;
