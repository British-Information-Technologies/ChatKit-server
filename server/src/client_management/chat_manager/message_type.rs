use chrono::{DateTime, Local};

use uuid::Uuid;

pub struct Message {
	from: Uuid,
	content: String,
	time: DateTime<Local>,
}

impl Message {
	pub fn new(from: Uuid, content: String) -> Self {
		Self {
			from,
			content,
			time: Local::now(),
		}
	}
}
