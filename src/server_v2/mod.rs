pub mod client_v2;
pub mod commands_v2;

use client_v2::ClientV2;
use std::{
    collections::{HashMap, VecDeque},
    io,
    thread,
    sync::{
        mpsc::{channel, Sender, Receiver},
        Arc,
        Mutex,
    },
    ops::Deref,
    borrow::Borrow,
    time::Duration,
    net::TcpListener,
    io::Read,
};
use crate::server_v2::commands_v2::Commandsv2;
use crate::lib::ThreadPool;
use std::sync::mpsc::TryRecvError;
use crate::server_v2::commands_v2::Commandsv2::Disconnect;

enum server_message {
    start,
    stop,
    kick(String),
}

pub struct Serverv2 {
    name: String,
    host: String,
    owner: String,

    rx: Arc<Mutex<Receiver<server_message>>>,
    tx: Arc<Mutex<Sender<server_message>>>,

    connected_clients: Arc<Mutex<HashMap<String, ClientV2>>>,
    thread_pool: ThreadPool,
}

impl Serverv2 {
    pub fn new(name: String, host: String, owner: String) -> Serverv2 {

        let (tx,rx) = channel();

        Serverv2 {
            name,
            host,
            owner,

            rx: Arc::new(Mutex::new(rx)),
            tx: Arc::new(Mutex::new(tx)),

            connected_clients: Arc::new(Mutex::new(HashMap::new())),
            thread_pool: ThreadPool::new(16)
        }
    }

    pub fn start(&self) -> Result<(), io::Error> {
        let listener = TcpListener::bind("0.0.0.0:6001")?;




        // accepting clients
        thread::spawn(move || {
            match rx.lock().unwrap().try_recv() {
                Ok(a) => {}
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => {
                    self.connected_clients.lock()
                        .unwrap()
                        .iter()
                        .map(|(id, client)| {
                            let tx = client.get_tx();
                            tx.send(Disconnect(None));
                        });
                }
            }

        });

        Ok(())
    }

    pub fn stop(&self) {
    }

    pub fn add_client(&self, client: ClientV2) -> Result<(), &str> {
        let mut client_map = self.connected_clients.lock().unwrap();
        if client_map.contains_key(client.uuid.as_str()) {
            return Err("!exists:");
        }

        client_map.insert(client.uuid.to_string(), client);

        self.thread_pool.execute(|| {client.run()});
        Ok(())
    }

    pub fn get_tx(&self, mesage: server_message) {
        self.tx.clone();
    }

    fn log(mesaage: &str) {
        println!("Server: {}", mesaage);
    }
}