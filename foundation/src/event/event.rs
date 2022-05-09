use crate::event::event_result::EventResultBuilder;
use crate::event::EventResult;
use crate::event::EventResultType;
use std::collections::HashMap;

use futures::channel::oneshot::{channel, Receiver, Sender};

/// # Eventw
/// Object that holds details about an event being passed through the application.
///
/// ## Properties
/// - r#type: The event type
/// - args: A hashmap of arguments to be carried by the event
/// - sender: The sender to send the result for the event.
/// - receiver: The reciever of the event result from the event.
pub struct Event<T>
where
	T: Sync + Send,
{
	pub r#type: T,
	args: HashMap<String, String>,
	sender: Sender<EventResult>,
	receiver: Option<Receiver<EventResult>>,
}

impl<T> Event<T>
where
	T: Sync + Send,
{
	/// Fetches an argument from the arguments of the event.
	pub fn get_arg(&self, key: String) -> Option<String> {
		self.args.get(&key).cloned()
	}

	/// Creates an event result using the sender of the event.
	/// This consumes the event.
	pub fn respond(self, result_type: EventResultType) -> EventResultBuilder {
		EventResult::create(result_type, self.sender)
	}

	/// Used to await the result of the event if required.
	pub fn get_reciever(&mut self) -> Receiver<EventResult> {
		self.receiver.take().unwrap()
	}
}

pub struct EventBuilder<T> {
	#[allow(dead_code)]
	r#type: T,

	#[allow(dead_code)]
	args: HashMap<String, String>,

	#[allow(dead_code)]
	sender: Sender<EventResult>,

	#[allow(dead_code)]
	receiver: Option<Receiver<EventResult>>,
}

impl<T> EventBuilder<T> {
	#[allow(dead_code)]
	pub(super) fn new(r#type: T) -> EventBuilder<T> {
		let (sender, receiver) = channel();
		EventBuilder {
			r#type,
			args: HashMap::new(),
			sender,
			receiver: Some(receiver),
		}
	}

	pub fn add_arg<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
		self.args.insert(key.into(), value.into());
		self
	}

	#[allow(dead_code)]
	pub(crate) fn build(self) -> Event<T>
	where
		T: Sync + Send,
	{
		Event {
			r#type: self.r#type,
			args: self.args,
			sender: self.sender,
			receiver: self.receiver,
		}
	}
}
