use actix::{Actor, Context, Handler};
use uuid::Uuid;

use crate::client_management::chat_manager::{
	message_type::Message, messages::ChatManagerMessage,
};

struct ChatManager {
	messages: Vec<Message>,
}

impl ChatManager {
	pub fn new() -> Self {
		Self {
			messages: Vec::new(),
		}
	}

	// no need for a remove methods because this is a read only system
	pub fn add_message(&mut self, id: Uuid, content: String) {
		self.messages.push(Message::new(id, content))
	}
}

impl Actor for ChatManager {
	type Context = Context<Self>;
}

impl Handler<ChatManagerMessage> for ChatManager {
	type Result = ();

	fn handle(
		&mut self,
		msg: ChatManagerMessage,
		_ctx: &mut Self::Context,
	) -> Self::Result {
		match msg {
			ChatManagerMessage::AddMessage(id, content) => self.add_message(id, content),
		}
	}
}
