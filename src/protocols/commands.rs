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
pub mod network;

use crate::client_management::client_profile::Client;
use std::collections::VecDeque;
use parking_lot::FairMutex;
use std::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;
use std::io::{self, Read};
use std::net::TcpStream;
use std::time::Duration;

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
    pub fn execute(&self, mut stream: &TcpStream, buffer: &mut [u8; 1024], data: &Vec<String>, address: &String, clients_ref: &Arc<Mutex<HashMap<String,Client>>>, message_queue: &Arc<FairMutex<VecDeque<String>>>){
        match *self{
            Commands::Info => {
                let server_details = info::get_server_info();

                let out_success = OutboundReturns::Success;
                out_success.execute(stream, &server_details);
            },
            Commands::Connect => {
                connect::add_client(clients_ref, &data[1], &data[2], address);

                let mut message = "!client: ".to_string();
                message.push_str(&data[2].to_string());
                message.push_str(&" address:".to_string());
                message.push_str(&address.to_string());
                message.push_str(&" ".to_string());
                message.push_str(&data[1].to_string());

                message_queue.lock().push_back(message);
                
                let out_success = OutboundReturns::Success;
                out_success.execute(stream, &String::from(""));
            },
            Commands::Disconnect => {
                let client_profile = disconnect::remove_client(clients_ref, &data[1]);

                let mut message = "!clientRemove: ".to_string();
                message.push_str(&client_profile.get_uuid().to_string());

                message_queue.lock().push_back(message);

                let out_success = OutboundReturns::Success;
                out_success.execute(stream, &String::from(""));
            },
            Commands::ClientUpdate => {
                let in_success = InboundReturns::Success;

                let clients_hashmap = clients_ref.lock().unwrap();
                for (key, value) in clients_hashmap.iter(){
                    let formatted_data = client_update::format_client_data(&key, &value);
                    network::transmit_data(stream, &formatted_data);

                    in_success.execute(stream, buffer, &formatted_data);
                }

                let out_success = OutboundReturns::Success;
                out_success.execute(stream, &String::from(""));
        
                in_success.execute(stream, buffer, &String::from("!success:"));
            },
            Commands::ClientInfo => {
                let requested_data = client_info::get_client_data(clients_ref, &data[1]);
                network::transmit_data(stream, &requested_data);
            },
            Commands::Unknown => {
                println!("Uknown Command!");
            },
        }
    }
}

impl OutboundCommands{
    pub fn execute(&self, mut stream: &TcpStream, buffer: &mut [u8; 1024], data: &String){
        match *self{
            OutboundCommands::Client => {
                network::transmit_data(stream, data);

                let in_success = InboundReturns::Success;
                in_success.execute(stream, buffer, data);
            },
            OutboundCommands::ClientRemove => {
                network::transmit_data(stream, data);

                let in_success = InboundReturns::Success;
                in_success.execute(stream, buffer, data);
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
                                    network::transmit_data(stream, data);
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
    pub fn execute(&self, mut stream: &TcpStream, data: &String){
        match *self{
            OutboundReturns::Success => {
                let mut message = "!success:".to_string();
                if !data.is_empty(){
                    message.push_str(&" ".to_string());
                    message.push_str(&data.to_string());
                }
                network::transmit_data(stream, &message);
            },
            OutboundReturns::Error => {},
        }
    }
}
