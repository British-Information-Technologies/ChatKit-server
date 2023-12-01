use std::collections::HashMap;

use futures::channel::oneshot::Sender;

pub enum EventResultType {
	Success,
	NoResponse,
	InvalidArgs,
	InvalidCode,
	Other(String),
}

pub struct EventResult {
	code: EventResultType,
	args: HashMap<String, String>,
}

impl EventResult {
	pub fn create(
		result_type: EventResultType,
		sender: Sender<EventResult>,
	) -> EventResultBuilder {
		EventResultBuilder::new(result_type, sender)
	}
}

/// # EventResultBuilder
/// Builds the result of an event
pub struct EventResultBuilder {
	code: EventResultType,
	args: HashMap<String, String>,
	sender: Sender<EventResult>,
}

impl EventResultBuilder {
	pub(self) fn new(
		result_type: EventResultType,
		sender: Sender<EventResult>,
	) -> Self {
		Self {
			code: result_type,
			args: HashMap::default(),
			sender,
		}
	}

	pub fn add_arg(mut self, key: String, value: String) -> Self {
		self.args.insert(key, value);
		self
	}

	pub fn send(self) {
		self
			.sender
			.send(EventResult {
				code: self.code,
				args: self.args,
			})
			.ok();
	}
}
