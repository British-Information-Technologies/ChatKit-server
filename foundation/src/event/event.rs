use crate::event::EventResult;
use std::collections::HashMap;
use uuid::Uuid;

use futures::channel::oneshot::{channel, Receiver, Sender};

//todo: move this out of foundation
pub enum EventType<'a> {
	NewConnection,
	ClientAdded,
	Custom(&'a str),
}

pub struct Event<T> {
	pub r#type: T,
	args: HashMap<String, String>,
	sender: Sender<EventResult>,
	receiver: Option<Receiver<EventResult>>,
}

pub struct EventBuilder<T> {
	r#type: T,
	args: HashMap<String, String>,
	sender: Sender<EventResult>,
	receiver: Option<Receiver<EventResult>>,
}

impl<T> EventBuilder<T> {
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

	pub(crate) fn build(self) -> Event<T> {
		Event {
			r#type: self.r#type,
			args: self.args,
			sender: self.sender,
			receiver: self.receiver,
		}
	}
}
