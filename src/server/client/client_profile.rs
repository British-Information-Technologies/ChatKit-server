extern crate regex;

use crate::server::server_profile::Server;
use crate::server::commands::{Commands};

use std::net::{Shutdown, TcpStream};
use std::sync::Arc;
use parking_lot::FairMutex;
use std::collections::HashMap;
use dashmap::DashMap;
use std::io::prelude::*;
use std::time::Duration;
use std::io::Error;
use crossbeam::{Sender, Receiver, TryRecvError};
use crossbeam_channel::unbounded;


pub struct Client<'a> {
    connected: bool,
    stream: Arc<TcpStream>,
    uuid: String,
    username: String,
    address: String,
    server: &'a Server<'a>,
    tx_channel: Sender<Commands>,
    rx_channel: Receiver<Commands>,
}

impl<'a> Client<'a> {
    pub fn new(server: &'a Server<'static>, stream: Arc<TcpStream>, uuid: &String, username: &String, address: &String) -> Self{
        let (tx_channel, rx_channel): (Sender<Commands>, Receiver<Commands>) = unbounded();

        Client {
            connected: true,
            stream,
            uuid: uuid.to_string(),
            username: username.to_string(),
            address: address.to_string(),
            server,
            tx_channel,
            rx_channel,
        }
    }

    #[allow(dead_code)]
    fn get_stream(&self) -> &TcpStream{
        &self.stream
    }

    #[allow(dead_code)]
    pub fn get_transmitter(&self) -> &Sender<Commands>{
        &self.tx_channel
    }
    
    #[allow(dead_code)]
    pub fn get_uuid(&self) -> &String{
        &self.uuid
    }

    #[allow(dead_code)]
    pub fn get_username(&self) -> &String{
        &self.username
    }

    #[allow(dead_code)]
    pub fn get_address(&self) -> &String{
        &self.address
    }

    pub fn handle_connection(&mut self){
        self.stream.set_read_timeout(Some(Duration::from_millis(2000))).unwrap();
        //self.stream.set_nonblocking(true).expect("set_nonblocking call failed");
        
        while self.connected {
            match self.rx_channel.try_recv() {
                /*command is on the channel*/
                Ok(command) => {
                    let a = command.clone();
                    match command {
                        /*this might be useless*/
                        Commands::Info(Some(_params)) => {
                            self.transmit_data(a.to_string().as_str());
                        },
                        Commands::Disconnect(None) => {
                            
                        },
                        Commands::ClientUpdate(Some(params)) => {
                            let uuid = params.get("uuid").unwrap();
                            
                            let data: HashMap<String, String> = [(String::from("uuid"), self.uuid.clone()), (String::from("name"), self.username.clone()), (String::from("host"), self.address.clone())].iter().cloned().collect();
                            let command = Commands::Client(Some(data));

                            self.server.update_client(uuid.as_str(), &command);
                        },
                        Commands::ClientInfo(Some(params)) => {
                            let uuid = params.get("uuid").unwrap();

                            let data: HashMap<String, String> = [(String::from("uuid"), self.uuid.clone()), (String::from("name"), self.username.clone()), (String::from("host"), self.address.clone())].iter().cloned().collect();
                            let command = Commands::Success(Some(data));

                            self.server.update_client(uuid.as_str(), &command);
                        },
                        Commands::ClientRemove(Some(params)) => {
                            let command = Commands::Client(Some(params));
                            self.transmit_data(command.to_string().as_str());

                            self.confirm_success();
                        },
                        Commands::Client(Some(_params)) => {
                            self.transmit_data(a.to_string().as_str());

                            self.confirm_success();
                        },
                        Commands::Success(_params) => {
                            self.transmit_data(a.to_string().as_str());
                        },
                        _ => {
                            println!("---Invalid Channel Command---");
                        },
                    }
                },
                /*sender disconnected*/
                Err(TryRecvError::Disconnected) => {},
                /*no data available yet*/
                Err(TryRecvError::Empty) => {},
            }

            /*
             * if multiple commands are written to the stream before it reads, all the commands
             * could be read at once, causing the program to ignore all commands after the firet
             * one. Ethier make sure commands sent require a response before sending the next one
             * or make a system to check for these issues.
             */
            match self.read_data() {
                Ok(command) => {
                    match command {
                        Commands::Info(None) => {
                            let params: HashMap<String, String> = [(String::from("name"), self.server.get_name()), (String::from("owner"), self.server.get_author())].iter().cloned().collect();
                            let command = Commands::Success(Some(params));
                            
                            self.transmit_data(command.to_string().as_str());
                        },
                        Commands::Disconnect(None) => {
                            self.connected = false;

                            self.server.remove_client(self.uuid.as_str());

                            self.stream.shutdown(Shutdown::Both).expect("shutdown call failed");

                            let params: HashMap<String, String> = [(String::from("uuid"), self.uuid.clone())].iter().cloned().collect();
                            let command = Commands::ClientRemove(Some(params));
                            self.server.update_all_clients(self.uuid.as_str(), command);
                        },
                        Commands::ClientUpdate(None) => {
                            let mut command = Commands::Success(None);
                            self.transmit_data(command.to_string().as_str());

                            let data: HashMap<String, String> = [(String::from("uuid"), self.uuid.clone())].iter().cloned().collect();
                            command = Commands::ClientUpdate(Some(data));

                            self.server.update_all_clients(self.uuid.as_str(), command);
                        },
                        Commands::ClientInfo(Some(params)) => {
                            let uuid = params.get("uuid").unwrap();

                            let data: HashMap<String, String> = [(String::from("uuid"), self.uuid.clone())].iter().cloned().collect();
                            let command = Commands::ClientInfo(Some(data));

                            self.server.update_client(uuid.as_str(), &command);
                        },
                        Commands::Error(None) => {
                        },
                        _ => {
                            println!("---Invalid Command---");
                        },
                    }
                },
                Err(_) => {
                    //println!("no data read");
                },
            }
        }
        println!("---Thread Exit---");
    }

    pub fn transmit_data(&self, data: &str){
        println!("Transmitting...");
        println!("{} data: {}", self.uuid, data);

        self.get_stream().write(data.to_string().as_bytes()).unwrap();
        self.get_stream().flush().unwrap();
    }

    fn read_data(&self) -> Result<Commands, Error> {
        let mut buffer = [0; 1024];

        self.get_stream().read(&mut buffer)?;
        let command = Commands::from(&buffer);

        Ok(command)
    }

    fn confirm_success(&self){
        //self.stream.set_nonblocking(false).expect("set_nonblocking call failed");
        //self.stream.set_read_timeout(Some(Duration::from_millis(3000))).expect("set_read_timeout call failed");

        match self.read_data() {
            Ok(command) => {
                match command {
                    Commands::Success(_params) => {
                        println!("Success Confirmed");
                    },
                    _ => {
                        let error = Commands::Error(None);
                        self.transmit_data(error.to_string().as_str());
                    },
                }
            },
            Err(_) => {
                println!("no success read");
                let error = Commands::Error(None);
                self.transmit_data(error.to_string().as_str());
            },
        }

        //self.stream.set_read_timeout(None).expect("set_read_timeout call failed");
        //self.stream.set_nonblocking(true).expect("set_nonblocking call failed");
    }

    #[deprecated(since="24.7.20", note="will be removed in future, please do not use!")]
    #[allow(dead_code)]
    pub fn disconnect(&mut self){
        self.stream.shutdown(Shutdown::Both).expect("shutdown call failed");
        self.connected = false;
    }

    #[deprecated(since="24.7.20", note="will be removed in future, please do not use!")]
    #[allow(dead_code)]
    pub fn transmit_success(&self, data: &String){
        let mut success_message = "!success:".to_string();
        if !data.is_empty(){
            success_message.push_str(&" ".to_string());
            success_message.push_str(&data.to_string());
        }
        self.transmit_data(&success_message);
    }
    
    #[deprecated(since="24.7.20", note="will be removed in future, please do not use!")]
    #[allow(dead_code)]
    fn transmit_error(&self, data: &String){
        let mut error_message = "!error:".to_string();
        if !data.is_empty(){
            error_message.push_str(&" ".to_string());
            error_message.push_str(&data.to_string());
        }
        self.transmit_data(&error_message);
    }
}
