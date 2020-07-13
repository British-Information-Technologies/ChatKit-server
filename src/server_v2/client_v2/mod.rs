use std::string::ToString;
use std::sync::{Arc, Mutex, Weak};
use std::net::TcpStream;
use crate::server_v2::Serverv2;
use std::sync::mpsc::{Receiver, Sender, channel, TryRecvError};
use crate::server_v2::commands_v2::Commandsv2;

#[derive(Clone)]
pub struct ClientV2 {
    pub uuid: String,
    pub username: String,
    pub address: String,

    stream: Arc<Mutex<TcpStream>>,
    server_reference: Weak<Serverv2>,

    tx: Sender<Commandsv2>,
    rx: Receiver<Commandsv2>,
}

impl ClientV2 {
    pub fn new(stream: Arc<Mutex<TcpStream>>, server: Arc<Serverv2>, uuid: &String, username: &String, address: &String) -> ClientV2 {

        let (tx, rx) = channel();

        ClientV2 {
            stream: stream,
            server_reference: Arc::downgrade(&server),

            tx,
            rx,

            uuid: uuid.to_string(),
            username: username.to_string(),
            address: address.to_string(),

        }
    }

    pub fn run(&self) {



        loop {
            match self.rx.try_recv() {
                Ok(Command) => {

                }
                Err(TryRecvError::Empty) => { }
                Err(TryRecvError::Disconnected) => {

                }
            }
        }
    }

    pub fn get_tx(&self) -> Sender<Commandsv2> {
        self.tx.clone()
    }
}

