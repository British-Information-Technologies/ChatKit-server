extern crate regex;

use crate::server::client::client_profile::Client;
use crate::server::commands::{Commands};

use rust_chat_server::ThreadPool;
use std::net::{TcpStream, TcpListener};
use std::sync::{Arc, Mutex};
use crossbeam_channel::Sender;
use parking_lot::FairMutex;
use std::collections::HashMap;
use dashmap::DashMap;
use std::io::prelude::*;
use regex::Regex;

pub struct Server<'z> {
    name: &'z str,
    address: &'z str,
    author: &'z str,
    connected_clients: Arc<Mutex<HashMap<String, Sender<Commands>>>>,
    thread_pool: ThreadPool,
}

// MARK: - server implemetation
impl<'z> Server<'z> {
    pub fn new(name: &'z str, address: &'z str, author: &'z str) -> Self {
        Self {
            name: name,
            address: address,
            author: author,
            connected_clients: Arc::new(Mutex::new(HashMap::new())),
            thread_pool: ThreadPool::new(16),
        }
    }

    pub fn get_name(&self) -> String{
        self.name.to_string()
    }

    pub fn get_address(&self) -> String{
        self.address.to_string()
    }

    pub fn get_author(&self) -> String{
        self.author.to_string()
    }
 
    pub fn start(&'static self) {
        let listener = TcpListener::bind(self.get_address()).unwrap();
        let mut buffer = [0; 1024];

        loop {
            if let Ok((mut stream, addr)) = listener.accept() {
                println!("Server: new connection, {}", addr);

                let request = Commands::Request(None);
                self.transmit_data(&stream, &request.to_string().as_str());

                stream.read(&mut buffer).unwrap();
                let command = Commands::from(&buffer);
                
                match command {
                    Commands::Connect(Some(data)) => {
                        let uuid = data.get("uuid").unwrap();
                        let username = data.get("name").unwrap();
                        let address = data.get("host").unwrap();

                        let stream = Arc::new(stream);
                        let client = Client::new(self, stream, &uuid, &username, &address);
                        
                        let tx = client.get_transmitter();

                        let mut clients_hashmap = self.connected_clients.lock().unwrap();
                        clients_hashmap.insert(uuid.to_string(), tx.clone());
                        std::mem::drop(clients_hashmap);
                        
                        let success = Commands::Success(None);
                        tx.send(success).unwrap();

                        self.thread_pool.execute(move || {
                            client.handle_connection();
                        });

                        let params: HashMap<String, String> = [(String::from("name"), username.clone()), (String::from("host"), address.clone()), (String::from("uuid"), uuid.clone())].iter().cloned().collect();
                        let new_client = Commands::Client(Some(params));
                        
                        self.update_all_clients(new_client);
                    },
                    Commands::Info(None) => {
                        let params: HashMap<String, String> = [(String::from("name"), self.name.to_string().clone()), (String::from("owner"), self.author.to_string().clone())].iter().cloned().collect();
                        let command = Commands::Success(Some(params));
                        
                        self.transmit_data(&stream, command.to_string().as_str());
                    },
                    _ => {
                        println!("Invalid command!");
                        self.transmit_data(&stream, Commands::Error(None).to_string().as_str());
                    },
                }
            } 
        }
    }

    pub fn update_client(&self, uuid: &str, command: &Commands){
        let clients = self.connected_clients.lock().unwrap();
        let tx = clients.get(&uuid.to_string()).unwrap();
        tx.send(command.clone()).unwrap();
    }

    pub fn update_all_clients(&self, command: Commands){
        let clients = self.connected_clients.lock().unwrap();
        for tx in clients.values(){
            tx.send(command.clone()).unwrap();
        }
    }

    fn transmit_data(&self, mut stream: &TcpStream, data: &str){
        println!("Transmitting...");
        println!("data: {}",data);

        /*
         * This will throw an error and crash any thread, including the main thread, if
         * the connection is lost before transmitting. Maybe change to handle any exceptions
         * that may occur.
         */
        stream.write(data.to_string().as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    #[deprecated(since="24.7.20", note="will be removed in future, please do not use!")]
    #[allow(dead_code)]
    pub fn get_info(&self, tx: Sender<Commands>) {
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert(String::from("name"), self.name.to_string().clone());
        params.insert(String::from("owner"), self.author.to_string().clone());
        
        let command = Commands::Info(Some(params));
        tx.send(command).unwrap();
    }

    #[deprecated(since="24.7.20", note="will be removed in future, please do not use!")]
    #[allow(dead_code)]
    fn regex_data(&self, command_regex: &Regex, data: &str, command_addons: &mut HashMap<String, String>){
        for figure in command_regex.find_iter(data){
            let segment = figure.as_str().to_string();
            let contents: Vec<&str> = segment.split(":").collect();
            println!("key: {}, value: {}", contents[0].to_string(), contents[1].to_string());
            command_addons.insert(contents[0].to_string(), contents[1].to_string());
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use std::{thread, time};

    fn spawn_server(){
        thread::spawn(|| {
            lazy_static!{
                static ref SERVER_NAME: &'static str = "test";
                static ref SERVER_ADDRESS: &'static str = "0.0.0.0:6000";
                static ref SERVER_AUTHOR: &'static str = "test";
                static ref SERVER: Server<'static> = Server::new(&SERVER_NAME, &SERVER_ADDRESS, &SERVER_AUTHOR);
            }
            SERVER.start();
        });
    }

    #[test] 
    fn test_connect_command(){
        let mut buffer = [0; 1024];
        
        spawn_server();
        
        let millis = time::Duration::from_millis(2000);
        thread::sleep(millis);

        let mut stream = TcpStream::connect("0.0.0.0:6000").unwrap();

        stream.read(&mut buffer).unwrap();
        let mut command = Commands::from(&buffer);

        let msg = b"!connect: uuid:123456-1234-1234-123456 name:\"alice\" host:\"127.0.0.1\"";
        stream.write(msg).unwrap();

        stream.read(&mut buffer).unwrap();
        command = Commands::from(&buffer);

        assert_eq!(command, Commands::Success(None));
    }

    #[test]
    fn test_info_command(){
        let mut buffer = [0; 1024];

        spawn_server();
        
        let millis = time::Duration::from_millis(2000);
        thread::sleep(millis);

        let mut stream = TcpStream::connect("0.0.0.0:6000").unwrap();
        
        stream.read(&mut buffer).unwrap();
        let mut command = Commands::from(&buffer);
        
        let msg = b"!info:";
        stream.write(msg).unwrap();
        
        stream.read(&mut buffer).unwrap();
        command = Commands::from(&buffer);       

        let params: HashMap<String, String> = [(String::from("name"), String::from("test")), (String::from("owner"), String::from("test"))].iter().cloned().collect();
        assert_eq!(command, Commands::Success(Some(params)));
    }
}
