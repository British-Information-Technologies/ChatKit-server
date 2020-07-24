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
use regex::Regex;
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

    fn get_stream(&self) -> &TcpStream{
        &self.stream
    }

    pub fn get_transmitter(&self) -> &Sender<Commands>{
        &self.tx_channel
    }
    
    pub fn get_uuid(&self) -> &String{
        &self.uuid
    }

    pub fn get_username(&self) -> &String{
        &self.username
    }

    pub fn get_address(&self) -> &String{
        &self.address
    }

    pub fn handle_connection(&self){
        self.stream.set_read_timeout(Some(Duration::from_millis(3000))).unwrap();
        let mut buffer = [0; 1024];       
        
        while self.connected {
            match self.rx_channel.try_recv() {
                /*command is on the channel*/
                Ok(command) => {
                    let a = command.clone();
                    match command {
                        
                        Commands::Info(Some(_params)) => {
                            self.transmit_data(a.to_string().as_str());
                        },
                        Commands::Disconnect(None) => {
                            
                        },
                        Commands::ClientInfo(Some(params)) => {
                            let uuid = params.get("uuid").unwrap();

                            let data: HashMap<String, String> = [(String::from("uuid"), self.uuid.clone()), (String::from("name"), self.username.clone())].iter().cloned().collect();
                            let command = Commands::Success(Some(data));

                            self.server.update_client(uuid.as_str(), &command);
                        },
                        Commands::ClientRemove(Some(params)) => {
                        
                        },
                        Commands::Client(Some(_params)) => {
                            self.transmit_data(a.to_string().as_str());

                            self.get_stream().read(&mut buffer).unwrap();
                            let command = Commands::from(&buffer);
                            match command{
                                Commands::Success(_params) => {
                                    println!("sucess confirmed");
                                },
                                _ => {
                                    let error = Commands::Error(None);
                                    self.transmit_data(error.to_string().as_str());
                                },
                            }
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
            
            match self.stream.peek(&mut buffer){
                Ok(_) => {
                    self.get_stream().read(&mut buffer).unwrap();
                    let command = Commands::from(&buffer);
                    println!("Request: {}", command.to_string());

                    /*command behaviour*/
                    /* CAN RECV:
                     * info
                     * disconnect
                     * client update
                     * client info
                     * success
                     * error
                     *
                     * CANNOT RECV:
                     * connect
                     * request
                     * client
                     * error
                     */
                    match command {
                        Commands::Info(None) => {
                            let params: HashMap<String, String> = [(String::from("name"), self.server.get_name()), (String::from("owner"), self.server.get_author())].iter().cloned().collect();

                            let command = Commands::Info(Some(params));
                            
                            self.transmit_data(command.to_string().as_str());
                        },
                        Commands::Disconnect(None) => {
                        },
                        Commands::ClientUpdate(None) => {
                            let data: HashMap<String, String> = [(String::from("uuid"), self.uuid.clone())].iter().cloned().collect();
                            let command = Commands::ClientInfo(Some(data));

                            self.server.update_all_clients(command);
                        },
                        Commands::ClientInfo(Some(params)) => {
                            let uuid = params.get("uuid").unwrap();

                            let data: HashMap<String, String> = [(String::from("uuid"), self.uuid.clone())].iter().cloned().collect();
                            let command = Commands::ClientInfo(Some(data));
                            
                            self.server.update_client(uuid.as_str(), &command);
                        },
                        Commands::Success(_params) => {
                        },
                        Commands::Error(None) => {
                        },
                        _ => {
                            println!("---Invalid Command---");
                        },
                    }
                },
                Err(_) => {
                    println!("no data peeked");
                },
            }
        }
        println!("---Thread Exit---");
    }    

    pub fn transmit_data(&self, data: &str){
        println!("Transmitting...");
        println!("data: {}", data);

        self.get_stream().write(data.to_string().as_bytes()).unwrap();
        self.get_stream().flush().unwrap();
    }



    pub fn disconnect(&mut self){
        self.stream.shutdown(Shutdown::Both).expect("shutdown call failed");
        self.connected = false;
    }



    // deprecated
    pub fn confirm_success(&self, buffer: &mut [u8; 1024]){
        let success_regex = Regex::new(r###"!success:"###).unwrap();

        let _ = match self.get_stream().read(&mut *buffer) {
            Err(_error) => self.transmit_error(&String::from("")),
            Ok(_success) => {
                let incoming_message = String::from_utf8_lossy(&buffer[..]);
                if success_regex.is_match(&incoming_message){
                    println!("success");
                }else{
                    self.transmit_error(&String::from(""));
                }
            },
        };
    }

    pub fn transmit_success(&self, data: &String){
        let mut success_message = "!success:".to_string();
        if !data.is_empty(){
            success_message.push_str(&" ".to_string());
            success_message.push_str(&data.to_string());
        }
        self.transmit_data(&success_message);
    }
    
    fn transmit_error(&self, data: &String){
        let mut error_message = "!error:".to_string();
        if !data.is_empty(){
            error_message.push_str(&" ".to_string());
            error_message.push_str(&data.to_string());
        }
        self.transmit_data(&error_message);
    }
}
