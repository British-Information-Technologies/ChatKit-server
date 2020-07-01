mod request;
mod info;
mod success;
mod error;
mod connect;
mod disconnect;
mod client_update;
mod client_info;
mod client;
mod test;
mod message;

use crate::server::client::client_profile::Client;
use crate::server::utility;

use std::collections::VecDeque;
use parking_lot::FairMutex;
use std::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;
use std::io::{self, Read};
use std::net::TcpStream;
use std::time::Duration;
use dashmap::DashMap;

pub enum Commands{
    Info,
    Connect,
    Disconnect,
    ClientUpdate,
    ClientInfo,
    Unknown,
}

pub enum OutboundCommands{
    Client,
    ClientRemove,
    Unknown,
}

enum InboundReturns{
    Success,
    Error,
}

enum OutboundReturns{
    Success,
    Error,
}

impl Commands{
    pub fn execute(&self, client: &mut Client, buffer: &mut [u8; 1024], data: &HashMap<String, String>, clients_ref: &Arc<Mutex<HashMap<String, Client>>>, message_queue: &Arc<FairMutex<VecDeque<String>>>){
        let stream = client.get_stream();
        match *self{
            Commands::Info => {
                let server_details = info::get_server_info();

                let out_success = OutboundReturns::Success;
                out_success.execute(&stream, &server_details);
            },
            Commands::Connect => {
                connect::add_client(clients_ref, client);

                let mut message = "!client: username:".to_string();
                message.push_str(&client.get_username().to_string());
                message.push_str(&" host:".to_string());
                message.push_str(&client.get_address().to_string());
                message.push_str(&" uuid:".to_string());
                message.push_str(&client.get_uuid().to_string());

                message_queue.lock().push_back(message);
                
                let out_success = OutboundReturns::Success;
                out_success.execute(&stream, &String::from(""));
            },
            Commands::Disconnect => {
                disconnect::remove_client(clients_ref, client);

                let mut message = "!clientRemove: uuid:".to_string();
                message.push_str(&client.get_uuid().to_string());
                message_queue.lock().push_back(message);

                let out_success = OutboundReturns::Success;
                out_success.execute(&stream, &String::from(""));
                client.disconnect();
                println!("disconnected!");
            },
            Commands::ClientUpdate => {
                let in_success = InboundReturns::Success;

                let clients_hashmap = clients_ref.lock().unwrap();
                for (key, value) in clients_hashmap.iter(){
                    let formatted_data = client_update::format_client_data(&key, &value);
                    utility::transmit_data(&stream, &formatted_data);

                    in_success.execute(&stream, buffer, &formatted_data);
                }

                let out_success = OutboundReturns::Success;
                out_success.execute(&stream, &String::from(""));
        
                in_success.execute(&stream, buffer, &String::from("!success:"));
            },
            Commands::ClientInfo => {
                let requested_data = client_info::get_client_data(clients_ref, data);
                utility::transmit_data(&stream, &requested_data);
            },
            Commands::Unknown => {
                println!("Uknown Command!");
            },
        }
    }
}

impl OutboundCommands{
    pub fn execute(&self, client: &Client, buffer: &mut [u8; 1024], data: &HashMap<String, String>){
        let stream = client.get_stream();
        match *self{
            OutboundCommands::Client => {
                let mut message = String::from("");
                message.push_str(&data.get("command").unwrap());
                message.push_str(&" username:");
                message.push_str(&data.get("username").unwrap());
                message.push_str(&" host:");
                message.push_str(&data.get("host").unwrap());
                message.push_str(&" uuid:");
                message.push_str(&data.get("uuid").unwrap());

                utility::transmit_data(&stream, &message);

                let in_success = InboundReturns::Success;
                in_success.execute(&stream, buffer, &message);
            },
            OutboundCommands::ClientRemove => {
                let mut message = String::from("");
                message.push_str(&data.get("command").unwrap());
                message.push_str(&" uuid:");
                message.push_str(&data.get("uuid").unwrap());

                utility::transmit_data(&stream, &message);

                let in_success = InboundReturns::Success;
                in_success.execute(&stream, buffer, &message);
            },
            OutboundCommands::Unknown => {
                println!("Unknown Command!");
            },
        }
    }
}

impl InboundReturns{
    pub fn execute(&self, mut stream: &TcpStream, buffer: &mut [u8; 1024], data: &String){
        stream.set_read_timeout(Some(Duration::from_millis(3000))).unwrap();
        match *self{
            InboundReturns::Success => {
                let mut failing = true;
                while failing{
                    let _ = match stream.read(&mut *buffer){
                        Err(e) => {
                            match e.kind() {
                                io::ErrorKind::WouldBlock => {
                                    println!("Blocking...");
                                    utility::transmit_data(stream, data);
                                },
                                _ => panic!("Fatal Error {}", e),
                            }
                        },
                        Ok(m) => {
                            println!("{:?}", m);
                            failing = false;
                        },
                    };
                }
            },
            InboundReturns::Error => {},
        }
    }
}

impl OutboundReturns{
    pub fn execute(&self, stream: &TcpStream, data: &String){
        match *self{
            OutboundReturns::Success => {
                let mut message = "!success:".to_string();
                if !data.is_empty(){
                    message.push_str(&" ".to_string());
                    message.push_str(&data.to_string());
                }
                utility::transmit_data(stream, &message);
            },
            OutboundReturns::Error => {},
        }
    }
}
