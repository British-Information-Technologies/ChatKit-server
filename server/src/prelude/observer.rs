//! # observer.rs
//! crates a message type for the observer pattern.

use actix::{Message, WeakRecipient};

/// # ObservableMessage
/// represents common messages for observers
#[derive(Message)]
#[rtype(result = "()")]
pub enum ObservableMessage<M>
where
	M: Message + Send,
	M::Result: Send,
{
	Subscribe(WeakRecipient<M>),
	Unsubscribe(WeakRecipient<M>),
}
