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
