// use crate::lib::server::ServerMessages;
use foundation::prelude::IPreemptive;
use std::collections::HashMap;
use std::mem::replace;
use std::sync::Arc;
use std::sync::Mutex;

use crossbeam_channel::{unbounded, Receiver, Sender};
use uuid::Uuid;

use crate::client::Client;
use crate::messages::ClientMessage;
use crate::messages::ClientMgrMessage;
use crate::messages::ServerMessage;
use foundation::prelude::IMessagable;

/// # ClientManager
/// This struct manages all connected users
#[derive(Debug)]
pub struct ClientManager {
	clients: Mutex<HashMap<Uuid, Arc<Client>>>,

	server_channel: Mutex<Sender<ServerMessage>>,

	sender: Sender<ClientMgrMessage>,
	receiver: Receiver<ClientMgrMessage>,
}

impl ClientManager {
	pub fn new(server_channel: Sender<ServerMessage>) -> Arc<Self> {
		let (sender, receiver) = unbounded();

		Arc::new(ClientManager {
			clients: Mutex::default(),

			server_channel: Mutex::new(server_channel),

			sender,
			receiver,
		})
	}
}

impl IMessagable<ClientMgrMessage, Sender<ServerMessage>> for ClientManager {
	fn send_message(&self, msg: ClientMgrMessage) {
		self.sender.send(msg).unwrap();
	}
	fn set_sender(&self, sender: Sender<ServerMessage>) {
		let mut server_lock = self.server_channel.lock().unwrap();
		let _ = replace(&mut *server_lock, sender);
	}
}

impl IPreemptive for ClientManager {
	fn run(arc: &Arc<Self>) {
		loop {
			std::thread::sleep(std::time::Duration::from_secs(1));

			println!("[client manager]: Tick!");

			if !arc.receiver.is_empty() {
				for message in arc.receiver.iter() {
					use ClientMgrMessage::{Add, Remove, SendMessage};

					match message {
						Add(client) => {
							Client::start(&client);
							arc.clients
								.lock()
								.unwrap()
								.insert(client.uuid, client)
								.unwrap_or_default();
						}
						Remove(uuid) => {
							let _ = arc.clients.lock().unwrap().remove(&uuid);
						}
						SendMessage(to_uuid, from_uuid, content) => {
							let lock = arc.clients.lock().unwrap();
							if let Some(client) = lock.get(&to_uuid) {
								client.send_message(ClientMessage::Message(
									from_uuid, content,
								))
							}
						}
						#[allow(unreachable_patterns)]
						_ => println!("[Client manager]: not implemented"),
					}
				}
			}
		}
	}

	fn start(arc: &Arc<Self>) {
		let arc = arc.clone();
		std::thread::spawn(move || ClientManager::run(&arc));
	}
}
