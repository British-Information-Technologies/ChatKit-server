extern crate regex;

use std::{
    sync::Arc,
    net::{Shutdown, TcpStream},
    io::prelude::*,
};
use crossbeam::{Sender, Receiver, TryRecvError, unbounded};

use crate::{
    server::{
        server_profile::ServerMessages,
    },
    commands::Commands

};
use std::sync::Mutex;
use std::time::Duration;

#[derive(Debug)]
pub struct Client {

    pub uuid: String,
    pub username: String,
    pub address: String,

    stream_arc: Arc<Mutex<TcpStream>>,

    heartbeat_ticker: Arc<Mutex<u8>>,

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

            heartbeat_ticker: Arc::new(Mutex::new(5)),

            sender,
            receiver,

            server_sender,

        }
    }

    #[allow(unused_variables)]
    pub fn handle_connection(&self) {
        println!("buffer");
        let mut buffer = [0; 1024];

        // test to see if there is anything for the client to receive from its channel
        println!("{}: channel checks", self.uuid);
        match self.receiver.try_recv() {
            /*command is on the channel*/

            Ok(Commands::Info(Some(params))) => {
                self.transmit_data(Commands::Info(Some(params)).to_string().as_str());
            },

            Ok(Commands::Disconnect(None)) => {

            }

            Ok(Commands::ClientRemove(Some(params))) => { },
            Ok(Commands::Success(params)) => { self.transmit_data(Commands::Success(params).to_string().as_str()); },
            Ok(Commands::Client(Some(params))) => { self.transmit_data(Commands::Client(Some(params)).to_string().as_str()); },

            /*sender disconnected*/
            Err(TryRecvError::Disconnected) => {
                self.server_sender.send(ServerMessages::RequestDisconnect(self.uuid.clone()));
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

            stream.read(&mut buffer).unwrap();

            let command = Commands::from(&buffer);

            // match incomming commands
            println!("command");
            match command {
                Commands::Disconnect(None) => {
                    self.server_sender.send(ServerMessages::RequestDisconnect(self.uuid.clone())).expect("sending message to server failed");
                },
                Commands::HeartBeat(None) => {
                    self.transmit_data(Commands::HeartBeat(None).to_string().as_str())
                }
                _ => {

                    self.transmit_data(Commands::Error(None).to_string().as_str())
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

    pub fn transmit_data(&self, data: &str){
        println!("Transmitting data: {}", data);

        self.stream_arc.lock().unwrap().write_all(data.to_string().as_bytes()).unwrap();
        self.stream_arc.lock().unwrap().flush().unwrap();
    }
}
