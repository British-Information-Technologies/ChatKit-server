use actix::{Message, Recipient};

/// # ObservableMessage
/// represents common messages for observers
#[derive(Message)]
#[rtype(result = "()")]
pub enum ObservableMessage<M>
where
	M: Message + Send,
	M::Result: Send,
{
	Subscribe(Recipient<M>),
	Unsubscribe(Recipient<M>),
}
