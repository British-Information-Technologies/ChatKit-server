extern crate regex;

use crate::server::client::client_profile::Client;
use crate::server::commands::{ClientCommands, ServerCommands};

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

pub struct Server{
    name: String,
    address: String,
    author: String,
    connected_clients: Arc<Mutex<HashMap<String,Client>>>,
}

impl Server{
    pub fn new(name: &String, address: &String, author: &String, connected_clients: &Arc<Mutex<HashMap<String,Client>>>) -> Server{
        Server{
            name: name.to_string(),
            address: address.to_string(),
            author: author.to_string(),
            connected_clients: Arc::clone(&connected_clients),
        }
    }
    
    pub fn get_address(&self) -> &String{
        &self.address
    }

    pub fn get_info(&self) -> String{
        let mut server_details = "".to_string();
        server_details.push_str(&"name:".to_string());
        server_details.push_str(&self.name);
        server_details.push_str(&" owner:".to_string());
        server_details.push_str(&self.author);

        server_details
    }

    pub fn establish_connection(&self, mut stream: TcpStream) -> Result<Client, bool>{
        /*let listener = TcpListener::bind(self.address.clone()).unwrap();
        let pool = ThreadPool::new(10);
        //let (tx,rx): (Sender<Arc<Barrier>>, Receiver<Arc<Barrier>>) = unbounded();
        //let (clock_tx, _) = (tx.clone(), rx.clone()); 

        //stream.set_read_timeout(Some(Duration::from_millis(3000))).unwrap();
        loop{
            if let Ok((mut stream, addr)) = listener.accept(){
                println!("Connected: {}", addr);

                let connected_clients_ref = Arc::clone(&self.connected_clients);
                let request = String::from("?request:");
                self.transmit_data(&stream, &request);

                pool.execute(move || {*/
        let mut client_connection: Result<Client, bool> = Err(true);
        let mut buffer = [0; 1024];

        let request = String::from("?request:");
        self.transmit_data(&stream, &request);

        let mut stream = Arc::new(stream);
        while client_connection.is_err(){
            Arc::get_mut(&mut stream).unwrap().read(&mut buffer).unwrap();

            let incoming_message = String::from_utf8_lossy(&buffer[..]);
            let command = self.tokenize(&incoming_message);
            client_connection = match command{
                Ok(cmd) => {
                    match cmd{
                        ClientCommands::Connect(data) => {
                            //connecting = false;
                            let uuid = data.get("uuid").unwrap();
                            let username = data.get("name").unwrap();
                            let address = data.get("host").unwrap();

                            let stream = Arc::clone(&stream);
                            let mut client = Client::new(stream, &uuid, &username, &address);
                            client.connect(self, &self.connected_clients, &data);
                            //cmd.execute(&mut client, self, &mut buffer, &self.connected_clients);
                            Ok(client)
                            //client.handle_connection(self, &connected_clients_ref);
                        },
                        ClientCommands::Info => {
                            let server_details = self.get_info();
                            self.transmit_data(&stream, &server_details);
                            Err(true)
                        },
                        _ => {
                            println!("Invalid command!");
                            Err(true)
                        },
                    }
                },
                Err(e) => {
                    println!("{}", e);
                    Err(true)
                },
            };
        }
        client_connection
                /*});
            }
        }*/
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
