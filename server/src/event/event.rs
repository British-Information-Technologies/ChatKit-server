use std::collections::HashMap;

pub enum EventType<'str> {
	NewConnection,
	Custom(&'str str)
}

pub struct Event<'str> {
	Type: EventType<'str>,
	args: HashMap<&'str str, String>
}

pub struct Builder<'str> {
	Type: EventType<'str>,
	args: HashMap<&'str str, String>
}

impl<'str> Builder<'str> {
	pub(super) fn new(Type: EventType<'str>) -> Builder {
		Builder {
			Type,
			args: HashMap::new()
		}
	}

	pub fn add_arg<T: Into<String>>(mut self, key: &'str str, value: T) -> Self {
		self.args.insert(key, value.into());
		self
	}

	pub(crate) fn build(self) -> Event<'str> {
		Event {
			Type: self.Type,
			args: self.args
		}
	}
}
