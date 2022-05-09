use crate::client::Client;
use std::sync::Arc;
use uuid::Uuid;

pub enum EventType<'a> {
	NewConnection,
	// Todo: - change client to use traits
	ClientAdded(Uuid),
	Custom(&'a str),
}
