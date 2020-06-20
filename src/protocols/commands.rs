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
    Success,
    Error,
    Connect,
    Disconnect,
    ClientUpdate,
    ClientInfo,
    Unknown,
}

pub enum OutboundCommands{
    Success,
    Error,
    Client,
    ClientRemove,
    Unknown,
}

impl Commands{
    pub fn execute(&self, mut stream: &TcpStream, buffer: &mut [u8; 1024], data: &Vec<String>, address: &String, clients_ref: &Arc<Mutex<HashMap<String,Client>>>, message_queue: &Arc<FairMutex<VecDeque<String>>>){
        match *self{
            Commands::Info => {
                let server_details = info::get_server_info();

                let mut message = "!success: ".to_string();
                message.push_str(&server_details.to_string());

                network::transmit_data(stream, &message);
            },
            Commands::Success => {
            },
            Commands::Error => {
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
                network::transmit_data(stream, &String::from("!success:"));
            },
            Commands::Disconnect => {
                let client_profile = disconnect::remove_client(clients_ref, &data[1]);

                let mut message = "!clientRemove: ".to_string();
                message.push_str(&client_profile.get_uuid().to_string());

                message_queue.lock().push_back(message);
                network::transmit_data(stream, &String::from("!success:"));
            },
            Commands::ClientUpdate => {
                stream.set_read_timeout(Some(Duration::from_millis(3000))).unwrap();
                //let mut buffer = [0; 1024];
                let mut failing = true;

                let clients_hashmap = clients_ref.lock().unwrap();
                for (key, value) in clients_hashmap.iter(){
                    let formatted_data = client_update::format_client_data(&key, &value);
                    network::transmit_data(stream, &formatted_data);

                    while failing{
                        let _ = match stream.read(&mut *buffer){
                            Err(e) => {
                                match e.kind() {
                                    io::ErrorKind::WouldBlock => {
                                        println!("Blocking...");
                                        network::transmit_data(stream, &formatted_data);
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
                }
                
                let final_message = String::from("!success:");
                network::transmit_data(stream, &final_message);
        
                failing = true;
                while failing{
                    let _ = match stream.read(&mut *buffer){
                        Err(e) => {
                            match e.kind() {
                                io::ErrorKind::WouldBlock => {
                                    println!("Blocking...");
                                    network::transmit_data(stream, &final_message);
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
            OutboundCommands::Success => {},
            OutboundCommands::Error => {},
            OutboundCommands::Client => {
                stream.set_read_timeout(Some(Duration::from_millis(3000))).unwrap();
                network::transmit_data(stream, data);

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
            OutboundCommands::ClientRemove => {
                stream.set_read_timeout(Some(Duration::from_millis(3000))).unwrap();
                network::transmit_data(stream, data);

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
            OutboundCommands::Unknown => {
                println!("Unknown Command!");
            },
        }
    }
}

/*mod network{
    use std::net::TcpStream;
    use std::io::Write;

    pub fn transmit_data(mut stream: &TcpStream, data: &str){
        println!("Transmitting...");
        println!("data: {}",data);

        stream.write(data.to_string().as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}*/
