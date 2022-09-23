use chrono::{DateTime, Local};
use uuid::Uuid;

#[derive(Clone)]
pub struct Message {
	_id: Uuid,
	_from: Uuid,
	_content: String,
	_time: DateTime<Local>,
}

impl Message {
	pub fn new(from: Uuid, content: String) -> Self {
		Self {
			_id: Uuid::new_v4(),
			_from: from,
			_content: content,
			_time: Local::now(),
		}
	}
}
