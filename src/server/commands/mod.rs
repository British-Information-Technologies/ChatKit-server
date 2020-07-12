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
use crate::server::server_profile::Server;

use parking_lot::FairMutex;
use std::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;
use dashmap::DashMap;

#[derive(Clone)]
pub enum ClientCommands{
    Info,
    Connect(HashMap<String, String>),
    Disconnect,
    ClientUpdate,
    ClientInfo(HashMap<String, String>),
    Unknown,
}

#[derive(Clone)]
pub enum ServerCommands{
    Client(HashMap<String, String>),
    ClientRemove(HashMap<String, String>),
    Unknown,
}

impl ClientCommands{
    pub fn execute(&self, client: &mut Client, server: &Server, buffer: &mut [u8; 1024], connected_clients: &Arc<Mutex<HashMap<String, Client>>>){
        let stream = client.get_stream();
        match &*self{
            ClientCommands::Info => {
                let server_details = server.get_info();

                client.transmit_success(&server_details);
            },
            ClientCommands::Connect(data) => {
                connect::add_client(connected_clients, client);

                let new_client = ServerCommands::Client(data.clone());
                server.update_all_clients(&new_client);
                
                client.transmit_success(&String::from(""));
            },
            ClientCommands::Disconnect => {
                disconnect::remove_client(connected_clients, client);

                let mut data: HashMap<String, String> = HashMap::new();
                data.insert("uuid".to_string(), client.get_uuid().to_string());

                let old_client = ServerCommands::ClientRemove(data);
                server.update_all_clients(&old_client);

                client.transmit_success(&String::from(""));
                client.disconnect();
                println!("disconnected!");
            },
            ClientCommands::ClientUpdate => {
                let clients_hashmap = connected_clients.lock().unwrap();
                for (key, value) in clients_hashmap.iter(){
                    let formatted_data = client_update::format_client_data(&key, &value);
                    client.transmit_data(&formatted_data);

                    client.confirm_success(buffer, &formatted_data);
                }
                client.transmit_success(&String::from(""));
                client.confirm_success(buffer, &String::from("!success:"));
            },
            ClientCommands::ClientInfo(data) => {
                let requested_data = client_info::get_client_data(connected_clients, &data);
                client.transmit_data(&requested_data);
            },
            ClientCommands::Unknown => {
                println!("Unknown Command");
            },
        }
    }
}

impl ServerCommands{
    pub fn execute(&self, client: &mut Client, buffer: &mut [u8; 1024]){
        match &*self{
            ServerCommands::Client(data) => {
                let mut message = String::from("");
                message.push_str(&"!client: name:");
                message.push_str(&data.get("name").unwrap());
                message.push_str(&" host:");
                message.push_str(&data.get("host").unwrap());
                message.push_str(&" uuid:");
                message.push_str(&data.get("uuid").unwrap());

                client.transmit_data(&message);

                client.confirm_success(buffer, &message);
            },
            ServerCommands::ClientRemove(data) => {
                let mut message = String::from("");
                message.push_str(&"!client: uuid:");
                message.push_str(&data.get("uuid").unwrap());

                client.transmit_data(&message);

                client.confirm_success(buffer, &message);
            },
            ServerCommands::Unknown => {
                println!("Unknown Command!");
            },
        }
    }
}
