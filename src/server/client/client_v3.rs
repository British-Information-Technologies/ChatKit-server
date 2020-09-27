extern crate regex;

use std::{
    io,
    io::Error,
    io::prelude::*,
    net::{Shutdown, TcpStream},
    sync::Arc,
    //collections::HashMap,
    sync::Mutex,
    time::{Duration, Instant},
};

use crossbeam_channel::{
    Receiver,
    Sender,
    TryRecvError,
    unbounded
};
use log::info;

use crate::{
    commands::Commands,
    server::server_v3::ServerMessages,
};



#[derive(Debug)]
pub struct Client {
    uuid: String,
    username: String,
    address: String,

    last_heartbeat: Instant,

    stream: Arc<Mutex<TcpStream>>,

    pub sender: Sender<Commands>,
    receiver: Receiver<Commands>,

    server_sender: Sender<ServerMessages>,
}

impl Client {
    #[allow(dead_code)]
    pub fn new(stream: TcpStream, server_sender: Sender<ServerMessages>, uuid: &str, username: &str, address: &str) -> Self {
        let (sender, receiver): (Sender<Commands>, Receiver<Commands>) = unbounded();
        stream.set_read_timeout(Some(Duration::from_secs(1))).unwrap();

        Client {
            stream: Arc::new(Mutex::new(stream)),
            uuid: uuid.to_string(),
            username: username.to_string(),
            address: address.to_string(),

            sender,
            receiver,

            server_sender,

            last_heartbeat: Instant::now(),
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
    #[allow(dead_code)]
    pub fn handle_connection(&mut self) {
        let mut buffer = [0; 1024];

        // TODO: - Check heartbeat
        {
            //info!("heartbeat")
        }

        info!("{}: handling connection", self.uuid);
        match self.read_data(&mut buffer) {


            Ok(Commands::Disconnect(None)) => {
                self.server_sender.send(ServerMessages::Disconnect(self.uuid.clone())).expect("sending message to server failed");
                self.stream.lock().unwrap().shutdown(Shutdown::Both).expect("shutdown call failed");
            },

            Ok(Commands::HeartBeat(None)) => {
                self.last_heartbeat = Instant::now();
                self.send_data(Commands::Success(None).to_string().as_str());
            },

            Ok(Commands::ClientUpdate(None)) => {
                self.send_data(Commands::Success(None).to_string().as_str());
                let _ = self.server_sender.send(ServerMessages::RequestUpdate(self.stream.clone()));
            },

            Ok(Commands::ClientInfo(Some(params))) => {
                let uuid = params.get("uuid").unwrap();
                let _ = self.server_sender.send(ServerMessages::RequestInfo(uuid.clone(), self.stream.clone()));
            },

            Ok(Commands::Error(None)) => {
                self.send_data(Commands::Error(None).to_string().as_str());
            },

            _ => {
                self.send_data(Commands::Error(None).to_string().as_str());
            },

            Err(_) => {
                // No data was read
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
                        self.send_data(Commands::Error(None).to_string().as_str());
                        break 'retry_loop1
                    } else {
                        self.send_data(Commands::ClientRemove(Some(params.clone())).to_string().as_str());

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
                        self.send_data(Commands::Error(None).to_string().as_str());
                        break 'retry_loop2;
                    } else {
                        self.send_data(Commands::Client(Some(params.clone())).to_string().as_str());

                        if self.read_data(&mut buffer).unwrap_or(Commands::Error(None)) == Commands::Success(None) {
                            break 'retry_loop2;
                        } else {
                            retry -= 1;
                        }
                    }
                }

            },
            /*No data available yet*/
            Err(TryRecvError::Empty) => {},
            _ => {},
        }
        println!("---Client Thread Exit---");
    }

    // move into a drop perhaps
    #[allow(dead_code)]
    pub fn disconnect(&mut self){
        self.stream.lock().unwrap().shutdown(Shutdown::Both).expect("shutdown call failed");
    }

    #[allow(dead_code)]
    pub fn send_data(&self, data: &str) {
        println!("Transmitting data: {}", data);

        let error_result = self.stream.lock().unwrap().write_all(data.to_string().as_bytes());
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

    #[allow(dead_code)]
    fn read_data(&mut self, buffer: &mut [u8; 1024]) -> Result<Commands, Error> {
        let _ = self.stream.lock().unwrap().read(buffer)?;
        let command = Commands::from(buffer);

        Ok(command)
    }

}

impl ToString for Client {
    fn to_string(&self) -> std::string::String { todo!() }
}

impl Drop for Client {
    fn drop(&mut self) {
        let _ = self.stream.lock().unwrap().write_all(Commands::Disconnect(None).to_string().as_bytes());
        let _ = self.stream.lock().unwrap().shutdown(Shutdown::Both);
    }
}
