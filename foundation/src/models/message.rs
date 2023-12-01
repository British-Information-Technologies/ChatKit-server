use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
	id: Uuid,
	from: Uuid,
	content: String,
	time: DateTime<Local>,
}

impl Message {
	pub fn new(from: Uuid, content: String) -> Self {
		Self {
			id: Uuid::new_v4(),
			from,
			content,
			time: Local::now(),
		}
	}
}
