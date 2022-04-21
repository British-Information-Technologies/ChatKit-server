use crate::event::EventResult;
use std::collections::HashMap;

use futures::channel::oneshot::{channel, Receiver, Sender};

pub enum EventType {
	NewConnection,
	Custom(String),
}

pub struct Event {
	Type: EventType,
	args: HashMap<String, String>,
	sender: Sender<EventResult>,
	receiver: Option<Receiver<EventResult>>,
}

pub struct EventBuilder {
	Type: EventType,
	args: HashMap<String, String>,
	sender: Sender<EventResult>,
	receiver: Option<Receiver<EventResult>>,
}

impl EventBuilder {
	pub(super) fn new(Type: EventType) -> EventBuilder {
		let (sender, receiver) = channel();
		EventBuilder {
			Type,
			args: HashMap::new(),
			sender,
			receiver: Some(receiver),
		}
	}

	pub fn add_arg<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
		self.args.insert(key.into(), value.into());
		self
	}

	pub(crate) fn build(self) -> Event {
		Event {
			Type: self.Type,
			args: self.args,
			sender: self.sender,
			receiver: self.receiver,
		}
	}
}
