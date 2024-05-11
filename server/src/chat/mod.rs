use foundation::prelude::GlobalMessage;

pub struct ChatManager {
	messages: Vec<GlobalMessage>,
}

impl ChatManager {
	pub fn new() -> Self {
		Self {
			messages: Vec::new(),
		}
	}

	pub fn add_message(&mut self, message: GlobalMessage) {
		println!("[ChatManager] added new global message {:?}", message);
		self.messages.push(message);
	}

	pub fn get_messages(&mut self) -> Vec<GlobalMessage> {
		println!("[ChatManager] got all messages");
		self.messages.clone()
	}
}

impl Default for ChatManager {
	fn default() -> Self {
		Self::new()
	}
}
