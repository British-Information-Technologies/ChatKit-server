extern crate regex;

use crate::server::client::client_profile::Client;
use crate::server::commands::{ClientCommands, ServerCommands, Commands};

use rust_chat_server::ThreadPool;
use std::collections::VecDeque;
use std::net::{TcpStream, TcpListener};
use std::sync::{Arc, Barrier, Mutex};
use crossbeam_channel::{unbounded, Sender, Receiver};
use parking_lot::FairMutex;
use std::collections::HashMap;
use dashmap::DashMap;
use std::io::prelude::*;
use regex::Regex;

pub struct Server<'server_lifetime> {
    name: String,
    address: String,
    author: String,
    connected_clients: Arc<Mutex<HashMap<String,Client<'server_lifetime>>>>,
    thread_pool: ThreadPool,
}

// MARK: - server implemetation
impl Server{
    pub fn new<'server_lifetime>(name: &String, address: &String, author: &String) -> Server<'server_lifetime>{
        Server{
            name: name.to_string(),
            address: address.to_string(),
            author: author.to_string(),
            connected_clients: Arc::new(Mutex::new(HashMap::new())),
            thread_pool: ThreadPool::new(16)
        }
    }
    
    pub fn get_address(&self) -> &String{
        &self.address
    }

    pub fn start(&self) {
        let listener = TcpListener::bind(self.get_address()).unwrap();
        let mut buffer = [0; 1024];

        //stream.set_read_timeout(Some(Duration::from_millis(3000))).unwrap();
        loop {
            if let Ok((mut stream, addr)) = listener.accept() {
                println!("Connected: {}", addr);

                let request = Commands::Request(None);
                self.transmit_data(&stream, request.to_string().as_str());

                stream.read(&mut buffer).unwrap();

                let incoming_message = String::from_utf8_lossy(&buffer[..]);
                let result = Commands::from_string(incoming_message.as_str());
                match result{
                    Ok(command) => {
                        match command{
                            Commands::Connect(Some(data)) => {
                                let uuid = data.get("uuid").unwrap();
                                let username = data.get("name").unwrap();
                                let address = data.get("host").unwrap();

                                let stream = Arc::clone(&stream);
                                let mut client = Client::new(self, stream, &uuid, &username, &address);
                                
                                self.thread_pool.execute(move || {
                                    client.handle_connection();
                                });

                                let mut clients_hashmap = self.connected_clients.lock().unwrap();
                                clients_hashmap.insert(uuid, client.clone());
                            },
                            Commands::Info(None) => {
                                let params: HashMap<String, String> = HashMap::new();
                                params.insert("name", &self.name);
                                params.insert("owner", &self.owner);

                                let command = Commands::Info(Some(params));
                                
                                self.transmit_data(&stream, command.to_string().as_str());
                            },
                            _ => {
                                println!("Invalid command!");
                                self.transmit_data(&stream, Commands::Error(None).to_string.as_str());
                            },
                        }
                    },
                    Err(e) => {
                        println!("error: {:?}", e);
                        self.transmit_data(&stream, Commands::Error(None).to_string.as_str());
                    },
                }
            }
        }
    }

    pub fn get_info(&self, tx: Sender<Commands>) {
        let params: HashMap<String, String> = HashMap::new();
        params.insert("name", &self.name);
        params.insert("owner", &self.owner);
        
        let command = Commands::Info(Some(params));
        tx.send(command).unwrap();
    }

    pub fn update_all_clients(&self, notification: &ServerCommands){
        let clients = self.connected_clients.lock().unwrap();
        for client in clients.values(){
            let tx = client.get_transmitter();
            tx.send(notification.clone()).unwrap();
        }
    }

    fn transmit_data(&self, mut stream: &TcpStream, data: &str){
        println!("Transmitting...");
        println!("data: {}",data);

        stream.write(data.to_string().as_bytes()).unwrap();
        stream.flush().unwrap();
    }

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

    fn regex_data(&self, command_regex: &Regex, data: &str, command_addons: &mut HashMap<String, String>){
        for figure in command_regex.find_iter(data){
            let segment = figure.as_str().to_string();
            let contents: Vec<&str> = segment.split(":").collect();
            println!("key: {}, value: {}", contents[0].to_string(), contents[1].to_string());
            command_addons.insert(contents[0].to_string(), contents[1].to_string());
        }
    }
}
