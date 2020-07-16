extern crate regex;

use crate::server::server_profile::Server;
use crate::server::commands::{Commands};

use std::net::{Shutdown, TcpStream};
use std::sync::Arc;
use crossbeam_channel::{Receiver, TryRecvError, unbounded, Sender};
use parking_lot::FairMutex;
use std::collections::HashMap;
use dashmap::DashMap;
use std::io::prelude::*;
use std::time::Duration;
use regex::Regex;

#[derive(Clone)]
pub struct Client<'client_lifetime>{
    connected: bool,
    stream: Arc<TcpStream>,
    uuid: String,
    username: String,
    address: String,
    server: &'client_lifetime Server<'client_lifetime>,
    tx_channel: Sender<Commands>,
    rx_channel: Receiver<Commands>,
}

impl<'a> Client<'a> {
    pub fn new(server: &'a Server<'a>, stream: Arc<TcpStream>, uuid: &String, username: &String, address: &String) -> Client<'a>{
        let (tx_channel, rx_channel): (Sender<Commands>, Receiver<Commands>) = unbounded();

        Client {
            connected: true,
            stream: stream,
            uuid: uuid.to_string(),
            username: username.to_string(),
            address: address.to_string(),
            server: server,
            tx_channel: tx_channel,
            rx_channel: rx_channel,
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
                        
                        Commands::Info(Some(params)) => {
                            self.get_stream().write_all(a.to_string().as_bytes());
                        },
                        Commands::Disconnect(None) => {
                            
                        },
                        Commands::ClientRemove(Some(params)) => {},
                        Commands::Client(Some(params)) => {},
                        Commands::Success(data) => {},
                        _ => {},
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
                    
                    let incoming_message = String::from(String::from_utf8_lossy(&buffer));
                    let command = Commands::from(incoming_message.clone());

                    println!("Request: {}", &incoming_message);
                    
                    match command {
                        Commands::Connect(Some(params)) => todo!(),
                        _ => todo!(),
                    }
                },
                Err(_) => {
                    println!("no data peeked");
                },
            }
        }
        println!("---thread exit---");
    }    

    // deprecated
    /*
    pub fn connect(&self, server: &Server, connected_clients: &Arc<Mutex<HashMap<String, Client>>>, data: &HashMap<String, String>){
        let mut clients_hashmap = connected_clients.lock().unwrap();
        let uuid = self.get_uuid().to_string();
        clients_hashmap.insert(uuid, self.clone());
        std::mem::drop(clients_hashmap);

        let new_client = Commands::Client(data.clone());
        server.update_all_clients(&new_client);

        self.transmit_success(&String::from(""));
    }
    */

    pub fn disconnect(&mut self){
        self.stream.shutdown(Shutdown::Both).expect("shutdown call failed");
        self.connected = false;
    }

    pub fn transmit_data(&self, data: &str){
        println!("Transmitting...");
        println!("data: {}", data);

        self.get_stream().write(data.to_string().as_bytes()).unwrap();
        self.get_stream().flush().unwrap();
    }

    // deprecated
    pub fn confirm_success(&self, buffer: &mut [u8; 1024], data: &String){
        let success_regex = Regex::new(r###"!success:"###).unwrap();

        let _ = match self.get_stream().read(&mut *buffer) {
            Err(error) => self.transmit_error(&String::from("")),
            Ok(success) => {
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
