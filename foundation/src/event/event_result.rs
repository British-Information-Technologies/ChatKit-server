use futures::channel::oneshot::Sender;
use std::collections::HashMap;

pub enum EventResultType {
	Success,
	InvalidArgs,
	InvalidCode,
	Other(String),
}

pub struct EventResult {
	code: EventResultType,
	args: HashMap<String, String>,
}

impl EventResult {
	pub fn create(result_type: EventResultType) -> EventResultBuilder {
		EventResultBuilder::new(result_type)
	}
}

pub struct EventResultBuilder {
	code: EventResultType,
	args: HashMap<String, String>,
}

impl EventResultBuilder {
	pub(self) fn new(result_type: EventResultType) -> Self {
		Self {
			code: result_type,
			args: HashMap::default(),
		}
	}

	pub fn add_arg(mut self, key: String, value: String) -> Self {
		self.args.insert(key, value);
		self
	}

	pub fn build(self) -> EventResult {
		EventResult {
			code: self.code,
			args: self.args,
		}
	}

	pub fn send(self, sender: Sender<EventResult>) {
		sender
			.send(EventResult {
				code: self.code,
				args: self.args,
			})
			.ok();
	}
}
