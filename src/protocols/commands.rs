mod request;
mod info;
mod success;
mod error;
mod connect;
mod disconnect;
mod client_update;
mod client;
mod test;
mod message;

//use connect::Connect;
//use crate::protocols::commands::connect::Connect;
//use crate::protocols::commands::client_update::ClientUpdate;
//use crate::protocols::commands::client::ClientQ;

use crate::client_management::client_profile::Client;

use std::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;
use std::net::TcpStream;
use std::io::Write;

pub enum Commands{
    Request,
    Info,
    Success,
    Error,
    Connect,
    Disconnect,
    ClientUpdate,
    Client,
    Test,
    Message,
    Unknown,
}

impl Commands{
    pub fn execute(&self, stream: TcpStream, data: &Vec<String>, address: &String, clients_ref: &Arc<Mutex<HashMap<String,Client>>>){
        match *self{
            Commands::Request => {
            }
            Commands::Info => {
            }
            Commands::Success => {
            }
            Commands::Error => {
            }
            Commands::Connect => {
                connect::add_new_client(clients_ref, &data[1], &data[2], address);
            }
            Commands::Disconnect => {
            }
            Commands::ClientUpdate => {
                let address = client_update::get_client_address(clients_ref, &data[1]);
                Commands::transmit_data(stream, &address);
            }
            Commands::Client => {
                let address = client::retrieve_requested_clients(clients_ref, &data[1]);
                Commands::transmit_data(stream, &address);
            }
            Commands::Test => {
            }
            Commands::Message => {
            }
            Commands::Unknown => {
                println!("Uknown Command!");
            }
        }
    }

    fn transmit_data(mut stream: TcpStream, data: &str){
        println!("Transmitting...");
        println!("data: {}",data);

        stream.write(data.to_string().as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
