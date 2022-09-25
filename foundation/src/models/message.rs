use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
