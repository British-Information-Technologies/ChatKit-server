use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::thread::spawn;
use std::time::Duration;

use crossbeam_channel::Sender as CrossSender;
use cursive::backends::curses::n::ncurses::LcCategory::numeric;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{channel, Sender as TokioSender};
use tokio::sync::Mutex;
use tokio::time::sleep;

use foundation::ClientDetails;
use crate::{Cursive, TextView};
use crate::managers::NetworkManager;

pub enum WorkerMessage {
	Disconnect,
	Connect {username: String},
}

pub type CursiveSender = CrossSender<Box<dyn FnOnce(&mut Cursive) + Send>>;

pub struct Worker {
	cursive_sender: CursiveSender,
	
	network_manager: NetworkManager,
	
	number: Arc<Mutex<usize>>,
	
	user_details: Mutex<Option<ClientDetails>>,
}

impl Worker {
	pub fn new(sender: CursiveSender) -> Worker {
		Worker {
			cursive_sender: sender,
			number: Arc::new(Mutex::new(0)),
			user_details: Mutex::new(None)
		}
	}
	
	pub fn start(self) -> TokioSender<WorkerMessage> {
		let (tx,rx) = channel::<WorkerMessage>(16);
		spawn(move || {
			
			let sender = self.cursive_sender.clone();
			let rt  = Runtime::new().unwrap();
			let tmp_num = self.number.clone();
			rt.block_on(async move {
				let a = &tmp_num;
				loop {
					let num = Arc::clone(&a);
					sleep(Duration::new(1,0)).await;
					let _ = sender.send(Box::new( move |s| {
						let num = &num.clone();
						let mut num_lock = num.blocking_lock();
						*num_lock += 1;
						let a = *num_lock;
						s.find_name::<TextView>("TextView").unwrap().set_content(a.to_string());
					}));
				}
			})
		});
		tx
	}
	
	
	pub async fn sendLoginInfo(&self) {
	
	}
}