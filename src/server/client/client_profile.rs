use std::{
    io::prelude::*,
    sync::Arc,
    io,
    sync::Mutex,
    time::{Instant, Duration},
    net::{TcpStream, Shutdown}
};
use crossbeam::{
    Sender,
    Receiver,
    TryRecvError,
    unbounded
};
use log::info;

use crate::server::server_profile::ServerMessages;
use crate::commands::Commands;

#[derive(Debug)]
pub struct Client {

    pub uuid: String,
    pub username: String,
    pub address: String,

    last_heartbeat: Arc<Mutex<Instant>>,

    stream_arc: Arc<Mutex<TcpStream>>,

    pub sender: Sender<Commands>,
    receiver: Receiver<Commands>,

    server_sender: Sender<ServerMessages>,
}

impl Client {
    pub fn new(stream: TcpStream, server_sender: Sender<ServerMessages>, uuid: String, username: String, address: String) -> Self {
        let (sender, receiver): (Sender<Commands>, Receiver<Commands>) = unbounded();
        stream.set_read_timeout(Some(Duration::from_secs(1))).unwrap();

        Client {
            stream_arc: Arc::new(Mutex::new(stream)),
            uuid: uuid.to_string(),
            username: username.to_string(),
            address: address.to_string(),

            sender,
            receiver,

            server_sender,

            last_heartbeat: Arc::new(Mutex::new(Instant::now())),
        }
    }

    // TODO: - add heartbeat timer.
    pub fn handle_connection(&self) {
        info!("{}: handling connection", self.uuid);

        println!("buffer");
        let mut buffer = [0; 1024];

        // test to see if there is anything for the client to receive from its channel

        match self.receiver.try_recv() {
            /*command is on the channel*/

            Ok(Commands::ClientRemove(Some(params))) => {
                let retry: u8 = 3;
                'retry_loop1: loop {
                    if retry < 1 {
                        self.transmit_data(Commands::Error(None).to_string().as_str());
                        break 'retry_loop1
                    }
                    self.transmit_data(Commands::ClientRemove(Some(params.clone())).to_string().as_str());
                    let _ = self.stream_arc.lock().unwrap().read(&mut buffer);
                    let command = Commands::from(&buffer);
                    if command == Commands::Success(None) {
                        break 'retry_loop1;
                    }
                }
            },
            Ok(Commands::Client(Some(params))) => {
                let retry: u8 = 3;
                'retry_loop2: loop {
                    if retry < 1 {
                        self.transmit_data(Commands::Error(None).to_string().as_str());
                        break 'retry_loop2;
                    }
                    self.transmit_data(Commands::Client(Some(params.clone())).to_string().as_str());
                    let _ = self.stream_arc.lock().unwrap().read(&mut buffer);
                    let command = Commands::from(&buffer);
                    if command == Commands::Success(None) {
                        break 'retry_loop2;
                    }
                }

            },
            /*no data available yet*/
            Err(TryRecvError::Empty) => {},
            _ => {}
        }

        println!("socket");
        let a = self.stream_arc.lock().unwrap().peek(&mut buffer).is_ok();
        println!("does have content: {}", a);
        if self.stream_arc.lock().unwrap().peek(&mut buffer).is_ok() {
            let mut stream = self.stream_arc.lock().unwrap();

            let _ = stream.read(&mut buffer).unwrap();

            let command = Commands::from(&buffer);

            // match incomming commands
            println!("command");
            match command {
                Commands::Disconnect(None) => {
                    self.server_sender.send(ServerMessages::Disconnect(self.uuid.clone())).expect("sending message to server failed");
                },
                Commands::HeartBeat(None) => {
                    *self.last_heartbeat.lock().unwrap() = Instant::now();
                    let _ = stream.write_all(Commands::Success(None).to_string().as_bytes());
                },
                Commands::ClientUpdate(None) => {
                    let _ = self.server_sender.send(ServerMessages::RequestUpdate(self.stream_arc.clone()));
                    let _ = stream.write_all(Commands::Success(None).to_string().as_bytes());
                }
                _ => {
                    let _ = stream.write_all(Commands::Error(None).to_string().as_bytes());
                }
            }
        }
        println!("end");
    }    

    // move into a drop perhaps
    #[allow(dead_code)]
    pub fn disconnect(&mut self){
        self.stream_arc.lock().unwrap().shutdown(Shutdown::Both).expect("shutdown call failed");
    }

    pub fn transmit_data(&self, data: &str) {
        println!("Transmitting data: {}", data);

        let error_result = self.stream_arc.lock().unwrap().write_all(data.to_string().as_bytes());
        if let Some(error) = error_result.err(){
            match error.kind() {
                // handle disconnections
                io::ErrorKind::NotConnected => {
                    let _ = self.server_sender.send(ServerMessages::Disconnect(self.uuid.clone()));
                },
                _ => { }
            }
        }
    }
}

impl ToString for Client {
    fn to_string(&self) -> std::string::String { todo!() }
}

impl Drop for Client {
    fn drop(&mut self) {
        let _ = self.stream_arc.lock().unwrap().write_all(Commands::Disconnect(None).to_string().as_bytes());
        let _ = self.stream_arc.lock().unwrap().shutdown(Shutdown::Both);
    }
}
