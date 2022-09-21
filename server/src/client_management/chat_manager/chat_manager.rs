use actix::{Actor, Addr, Context, Handler};
use uuid::Uuid;

use crate::client_management::chat_manager::{
	message_type::Message,
	messages::{ChatManagerDataMessage, ChatManagerDataResponse, ChatManagerMessage},
};

pub(crate) struct ChatManager {
	messages: Vec<Message>,
}

impl ChatManager {
	pub fn new() -> Addr<Self> {
		Self {
			messages: Vec::new(),
		}
		.start()
	}

	// no need for a remove methods because this is a read only system
	fn add_message(&mut self, _ctx: &mut Context<Self>, id: Uuid, content: String) {
		self.messages.push(Message::new(id, content))
	}

	fn get_messages(&self, _ctx: &mut Context<Self>) -> ChatManagerDataResponse {
		ChatManagerDataResponse::GotMessages(self.messages.clone())
	}

	fn get_message(
		&self,
		_ctx: &mut Context<Self>,
		index: usize,
	) -> ChatManagerDataResponse {
		ChatManagerDataResponse::GotMessage(self.messages.get(index).cloned())
	}
}

impl Actor for ChatManager {
	type Context = Context<Self>;
}

impl Handler<ChatManagerMessage> for ChatManager {
	type Result = ();

	fn handle(&mut self, msg: ChatManagerMessage, ctx: &mut Self::Context) -> Self::Result {
		match msg {
			ChatManagerMessage::AddMessage(id, content) => self.add_message(ctx, id, content),
		}
	}
}

impl Handler<ChatManagerDataMessage> for ChatManager {
	type Result = ChatManagerDataResponse;

	fn handle(
		&mut self,
		msg: ChatManagerDataMessage,
		ctx: &mut Self::Context,
	) -> Self::Result {
		match msg {
			ChatManagerDataMessage::GetMessages => self.get_messages(ctx),
			ChatManagerDataMessage::GetMessage(index) => self.get_message(ctx, index),
		}
	}
}
