use crate::client::Client;
use crate::messages::ServerMessage;
use std::sync::{Arc, Weak};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::Mutex;

#[derive(Clone, Debug)]
pub struct Message {
	content: String,
	sender: Weak<Client<>>,
}

impl Message {
	#[allow(unused)]
	pub fn new(content: String, sender: Weak<Client<>>) -> Message {
		Message { content, sender }
	}
}

enum ChatManagerMessage {
	AddMessage {sender: Weak<Client<>>, content: String}
}

pub struct ChatManager {
	messages: Mutex<Vec<Message>>,
	server_channel: Sender<ServerMessage>,

	#[allow(unused)]
	tx: Sender<ChatManagerMessage>,
	rx: Mutex<Receiver<ChatManagerMessage>>,
}

impl ChatManager {
	#[allow(unused)]
	pub fn new(server_channel: Sender<ServerMessage>) -> Arc<Self> {
		let (tx, rx) = channel::<ChatManagerMessage>(1024);
		
		let manager = Arc::new(ChatManager {
			messages: Mutex::new(Vec::new()),
			server_channel,
			tx,
			rx: Mutex::new(rx),
		});

		manager.start();
		manager
	}

	#[allow(unused)]
	fn start(self: &Arc<ChatManager>) {
		let manager = self.clone();
		tokio::spawn(async move {
			use ServerMessage::{BroadcastGlobalMessage};
			use ChatManagerMessage::{AddMessage};
			
			while let Some(message) = manager.rx.lock().await.recv().await {
				
				match message {
					AddMessage { content,sender } => {
						let sender = &sender.upgrade().unwrap().details.uuid;
						manager.server_channel.send(
							BroadcastGlobalMessage {sender: sender.clone(), content}
						).await.unwrap();
					}
				}
			}
		});
	}

	#[allow(unused)]
	pub async fn add_message(self: &Arc<Self>, sender: Weak<Client>, content: String) {
		let mut a = self.messages.lock().await;
		a.push(Message::new(content, sender))
	}

	#[allow(unused)]
	pub async fn get_all_messages(self: &Arc<Self>) -> Vec<Message> {
		self.messages.lock().await.clone()
	}
}