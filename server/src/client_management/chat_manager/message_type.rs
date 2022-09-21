use chrono::{DateTime, Local};
use uuid::Uuid;

#[derive(Clone)]
pub struct Message {
	id: Uuid,
	from: Uuid,
	content: String,
	time: DateTime<Local>,
}

impl Message {
	pub fn new(from: Uuid, content: String) -> Self {
		Self {
			id: Uuid::new(),
			from,
			content,
			time: Local::now(),
		}
	}
}
