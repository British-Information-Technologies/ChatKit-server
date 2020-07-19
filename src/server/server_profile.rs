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
 
    pub fn get_address(&self) -> String{
        self.address.to_string()
    }

    pub fn start(&'static self) {
        let listener = TcpListener::bind(self.get_address()).unwrap();
        let mut buffer = [0; 1024];

        loop {
            if let Ok((mut stream, addr)) = listener.accept() {
                println!("Server: new connection, {}", addr);

                let request = Commands::Request(None);
                request.to_string();
                self.transmit_data(&stream, &*request.to_string().as_str());

                stream.read(&mut buffer).unwrap();

                let incoming_message = String::from(String::from_utf8_lossy(&buffer));
                let command = Commands::from(incoming_message);
                match command {
                    Commands::Connect(Some(data)) => {
                        let uuid = data.get("uuid").unwrap();
                        let username = data.get("name").unwrap();
                        let address = data.get("host").unwrap();

                        let stream = Arc::new(stream);
                        let mut client = Client::new(self, stream, &uuid, &username, &address);

                        let mut clients_hashmap = self.connected_clients.lock().unwrap();

                        clients_hashmap.insert(uuid.to_string(), client.get_transmitter().clone());

                        self.thread_pool.execute(move || {
                            client.handle_connection();
                        });

                        let params: HashMap<String, String> = [(String::from("name"), username.clone()), (String::from("host"), address.clone()), (String::from("uuid"), uuid.clone())].iter().cloned().collect();
                        let new_client = Commands::Client(Some(params));
                        
                        self.update_all_clients(new_client);
                    },
                    Commands::Info(None) => {
                        let mut params: HashMap<String, String> = HashMap::new();
                        params.insert(String::from("name"), self.name.to_string().clone());
                        params.insert(String::from("owner"), self.author.to_string().clone());

                        let command = Commands::Info(Some(params));
                        
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

    pub fn get_info(&self, tx: Sender<Commands>) {
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert(String::from("name"), self.name.to_string().clone());
        params.insert(String::from("owner"), self.author.to_string().clone());
        
        let command = Commands::Info(Some(params));
        tx.send(command).unwrap();
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

        stream.write(data.to_string().as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    //deprecated
    /*
    pub fn tokenize(&self, incoming_message: &str) -> Result<ClientCommands, &'static str>{
        let command_regex = Regex::new(r###"(\?|!)([a-zA-z0-9]*):|([a-zA-z]*):([a-zA-Z0-9\-\+\[\]{}_=/]+|("(.*?)")+)"###).unwrap();
        
        if command_regex.is_match(incoming_message){
            let command = self.match_command(&incoming_message.to_string());
            let command = match command{
                ClientCommands::Connect(mut addons) => {
                    self.regex_data(&command_regex, &incoming_message.replace("!connect: ", ""), &mut addons);
                    ClientCommands::Connect(addons)
                },
                ClientCommands::ClientInfo(mut addons) => {
                    self.regex_data(&command_regex, &incoming_message.replace("!clientInfo: ", ""), &mut addons);
                    ClientCommands::ClientInfo(addons)
                },
                _ => {
                    println!("no addons");
                    command
                },
            };
            Ok(command)
        } else {
            Err("data did not match regex!")
        }
    }
    
    
    fn match_command(&self, command: &String) -> ClientCommands{
        match command{
            _ if command.starts_with("!info:") => ClientCommands::Info,
            _ if command.starts_with("!connect:") => ClientCommands::Connect(HashMap::new()),
            _ if command.starts_with("!disconnect:") => ClientCommands::Disconnect,
            _ if command.starts_with("!clientUpdate:") => ClientCommands::ClientUpdate,
            _ if command.starts_with("!clientInfo:") => ClientCommands::ClientInfo(HashMap::new()),
            _ => ClientCommands::Unknown,
        }
    }
    */

    fn regex_data(&self, command_regex: &Regex, data: &str, command_addons: &mut HashMap<String, String>){
        for figure in command_regex.find_iter(data){
            let segment = figure.as_str().to_string();
            let contents: Vec<&str> = segment.split(":").collect();
            println!("key: {}, value: {}", contents[0].to_string(), contents[1].to_string());
            command_addons.insert(contents[0].to_string(), contents[1].to_string());
        }
    }
}
