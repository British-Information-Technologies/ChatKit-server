use actix::{Actor, Addr, Context, Handler};
use foundation::models::message::Message;
use uuid::Uuid;

use crate::client_management::chat_manager::messages::{
	ChatManagerDataMessage,
	ChatManagerDataResponse,
	ChatManagerMessage,
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
		println!(
			"[ChatManager] add_message id: {:?} content: {:?}",
			id, content
		);
		self.messages.push(Message::new(id, content))
	}

	fn get_messages(&self, _ctx: &mut Context<Self>) -> ChatManagerDataResponse {
		println!("[ChatManager] getting messages");
		ChatManagerDataResponse::GotMessages(self.messages.clone())
	}

	fn get_message(
		&self,
		_ctx: &mut Context<Self>,
		index: usize,
	) -> ChatManagerDataResponse {
		println!("[ChatManager] getting message index: {:?}", index);
		ChatManagerDataResponse::GotMessage(self.messages.get(index).cloned())
	}
}

impl Actor for ChatManager {
	type Context = Context<Self>;
}

impl Handler<ChatManagerMessage> for ChatManager {
	type Result = ();

	fn handle(&mut self, msg: ChatManagerMessage, ctx: &mut Self::Context) -> Self::Result {
		println!("[ChatManager] got message: {:?}", msg);
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
		println!("[ChatManager] got message: {:?}", msg);
		match msg {
			ChatManagerDataMessage::GetMessages => self.get_messages(ctx),
			ChatManagerDataMessage::GetMessage(index) => self.get_message(ctx, index),
		}
	}
}
