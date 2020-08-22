extern crate regex;

use std::{
    sync::Arc,
    sync::Mutex,
    net::{Shutdown, TcpStream},
    io::prelude::*,
    io::Error,
    //collections::HashMap,
    time::{Instant, Duration},
    io,
};

use crossbeam::{
    Sender,
    Receiver,
    TryRecvError,
    unbounded
};

//use zeroize::Zeroize;
use log::info;

use crate::{
    server::{
        //server_profile::Server,
        server_profile::ServerMessages,
    },
    commands::Commands

};

//use parking_lot::FairMutex;
//use dashmap::DashMap;

#[derive(Debug)]
pub struct Client {
    uuid: String,
    username: String,
    address: String,

    last_heartbeat: Arc<Mutex<Instant>>,

    stream_arc: Arc<Mutex<TcpStream>>,

    pub sender: Sender<Commands>,
    receiver: Receiver<Commands>,

    server_sender: Sender<ServerMessages>,
}

impl Client {
    pub fn new(stream: TcpStream, server_sender: Sender<ServerMessages>, uuid: &str, username: &str, address: &str) -> Self {
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

    #[allow(dead_code)]
    pub fn get_sender(&self) -> &Sender<Commands> {
        &self.sender
    }
    
    #[allow(dead_code)]
    pub fn get_uuid(&self) -> String {
        self.uuid.clone()
    }

    #[allow(dead_code)]
    pub fn get_username(&self) -> String {
        self.username.clone()
    }

    #[allow(dead_code)]
    pub fn get_address(&self) -> String {
        self.address.clone()
    }

    // TODO: - add heartbeat timer.
    pub fn handle_connection(&mut self) {
        let mut buffer = [0; 1024];

        // TODO: - Check heartbeat
        {
            info!("heartbeat")
        }
        
        info!("{}: handling connection", self.uuid);
        match self.read_data(&mut buffer) {
            Ok(command) => {
                // match incomming commands
                println!("command");
                match command {
                    Commands::Disconnect(None) => {
                        self.server_sender.send(ServerMessages::Disconnect(self.uuid.clone())).expect("sending message to server failed");
                        self.stream_arc.lock().unwrap().shutdown(Shutdown::Both).expect("shutdown call failed");
                    },
                    Commands::HeartBeat(None) => {
                        *self.last_heartbeat.lock().unwrap() = Instant::now();
                        self.transmit_data(Commands::Success(None).to_string().as_str());
                    },
                    Commands::ClientUpdate(None) => {
                        self.transmit_data(Commands::Success(None).to_string().as_str());
                        let _ = self.server_sender.send(ServerMessages::RequestUpdate(self.stream_arc.clone()));
                    },
                    Commands::ClientInfo(Some(params)) => {
                        let uuid = params.get("uuid").unwrap();
                        let _ = self.server_sender.send(ServerMessages::RequestInfo(uuid.clone(), self.stream_arc.clone()));
                    },
                    // TODO: may or may not be needed?
                    Commands::Error(None) => {
                    },
                    _ => {
                        self.transmit_data(Commands::Error(None).to_string().as_str());
                    },
                }
            },
            Err(_) => {
                // no data was read
            },
        }

        println!("buffer");
        // test to see if there is anything for the client to receive from its channel
        match self.receiver.try_recv() {
            /*command is on the channel*/ 
            Ok(Commands::ClientRemove(Some(params))) => {
                let mut retry: u8 = 3;
                'retry_loop1: loop {
                    if retry < 1 {
                        self.transmit_data(Commands::Error(None).to_string().as_str());
                        break 'retry_loop1
                    } else {                    
                        self.transmit_data(Commands::ClientRemove(Some(params.clone())).to_string().as_str());
                        
                        if self.read_data(&mut buffer).unwrap_or(Commands::Error(None)) == Commands::Success(None) {
                            break 'retry_loop1;
                        } else {
                            retry -= 1;
                        }
                    }
                }
            },
            Ok(Commands::Client(Some(params))) => {
                let mut retry: u8 = 3;
                'retry_loop2: loop {
                    if retry < 1 {
                        self.transmit_data(Commands::Error(None).to_string().as_str());
                        break 'retry_loop2;
                    } else {
                        self.transmit_data(Commands::Client(Some(params.clone())).to_string().as_str());
                        
                        if self.read_data(&mut buffer).unwrap_or(Commands::Error(None)) == Commands::Success(None) {
                            break 'retry_loop2;
                        } else {
                            retry -= 1;
                        }
                    }
                }

            },
            /*no data available yet*/
            Err(TryRecvError::Empty) => {},
            _ => {},
        }
        println!("---Client Thread Exit---");
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
                _ => { },
            }
        }
    }

    fn read_data(&mut self, buffer: &mut [u8; 1024]) -> Result<Commands, Error> {
        let _ = self.stream_arc.lock().unwrap().read(buffer)?;
        let command = Commands::from(buffer);

        Ok(command)
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
