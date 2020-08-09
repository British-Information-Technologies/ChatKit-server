use crate::{
    server::{
        client::client_profile::Client,

    },
    commands::Commands
};
use std::{
    sync::{Arc, Mutex},
    net::{TcpStream, TcpListener},
    collections::HashMap,
    io::prelude::*,
    thread,
    io
};


use log::info;

use crossbeam_channel::{Sender, Receiver, unbounded};
use rust_chat_server::ThreadPool;
use zeroize::Zeroize;
use std::time::Duration;

#[derive(Debug)]
pub enum ServerMessages {
    #[allow(dead_code)]
    RequestUpdate(Arc<Mutex<TcpStream>>),
    #[allow(dead_code)]
    RequestInfo(String, Arc<Mutex<TcpStream>>),
    #[allow(dead_code)]
    Disconnect(String),
    #[allow(dead_code)]
    Shutdown,
}

// MARK: - server struct

pub struct Server {
    pub name: String,
    pub address: String,
    pub author: String,

    connected_clients: Arc<Mutex<HashMap<String, Client>>>,

    thread_pool: ThreadPool,

    sender: Sender<ServerMessages>,
    receiver: Receiver<ServerMessages>,

}

// MARK: - server implemetation
impl Server {
    pub fn new(name: &str, address: &str, author: &str) -> Self {
        let (sender, receiver) = unbounded();

        Self {
            name: name.to_string(),
            address: address.to_string(),
            author: author.to_string(),
            connected_clients: Arc::new(Mutex::new(HashMap::new())),
            thread_pool: ThreadPool::new(16),


            sender,
            receiver,
        }
    }
 
    pub fn get_address(&self) -> String{
        self.address.to_string()
    }

    pub fn start<'a>(&self) -> Result<(), io::Error>{
        info!("server: starting server...");
        // clone elements for thread
        let client_map = self.connected_clients.clone();
        let sender = self.sender.clone();
        let receiver = self.receiver.clone();

        let server_details = (self.name.clone(), self.author.clone(), self.address.clone());

        // set up listener and buffer
        let listener = TcpListener::bind(self.get_address())?;
        listener.set_nonblocking(true)?;

        let mut buffer = [0; 1024];

        info!("server: spawning threads");
        let _ = thread::Builder::new().name("Server Thread".to_string()).spawn(move || {
            'outer: loop {
                // get messages from the servers channel.
                info!("server: getting messages");
                for i in receiver.try_iter() {
                    match i {
                        ServerMessages::Shutdown => {
                            // TODO: implement disconnecting all clients and shutting down the server
                            info!("server: shutting down...");

                            break 'outer;
                        },
                        ServerMessages::RequestUpdate(stream_arc) => {
                            let mut stream = stream_arc.lock().unwrap();
                            for (_k, v) in client_map.lock().unwrap().iter() {
                                let _ = &stream.write_all(v.to_string().as_bytes());
                                let _ = &stream.flush();
                            }
                        }
                        _ => {}
                    }
                }

                info!("server: checking for new connections");
                if let Ok((mut stream, _addr)) = listener.accept() {
                    stream.set_read_timeout(Some(Duration::from_millis(10000))).unwrap();

                    let request = Commands::Request(None);
                    //request.to_string();
                    let _ = stream.write_all(&request.to_string().as_bytes());
                    let _ = stream.flush();
                    let _ = stream.read(&mut buffer).unwrap();

                    let incoming_message = String::from(String::from_utf8_lossy(&buffer));
                    let command = Commands::from(incoming_message);
                    // clears the buffer.
                    buffer.zeroize();

                    match command {
                        Commands::Connect(Some(data)) => {
                            let uuid = data.get("uuid").unwrap();
                            let username = data.get("name").unwrap();
                            let address = data.get("host").unwrap();

                            info!("{}", format!("Server: new Client connection: _addr = {}", address ));

                            let client = Client::new(stream, sender.clone(), uuid.clone(), username.clone(), address.clone());

                            client_map.lock().unwrap().insert(uuid.to_string(), client);

                            let params: HashMap<String, String> = [(String::from("name"), username.clone()), (String::from("host"), address.clone()), (String::from("uuid"), uuid.clone())].iter().cloned().collect();
                            let new_client = Commands::Client(Some(params));

                            let _ = client_map.lock().unwrap().iter().map(|(_k, v)| v.sender.send(new_client.clone()));
                        },

                        // TODO: - correct connection reset error when getting info.
                        Commands::Info(None) => {
                            info!("Server: info requested");
                            let mut params: HashMap<String, String> = HashMap::new();
                            params.insert(String::from("name"), server_details.0.clone());
                            params.insert(String::from("owner"), server_details.1.clone());

                            let command = Commands::Info(Some(params));

                            stream.write_all(command.to_string().as_bytes()).unwrap();
                            stream.flush().unwrap();
                        },
                        _ => {
                            info!("Server: Invalid command sent");
                            let _ = stream.write_all(Commands::Error(None).to_string().as_bytes());
                            let _ = stream.flush();
                        },
                    }
                }
                // TODO: end -

                // handle each client for messages
                info!("server: handing control to clients");
                for (_k, v) in client_map.lock().unwrap().iter() {
                    v.handle_connection();
                }
            }
            info!("server: stopped");
        });
        info!("server: started");
        Ok(())
    }

    pub fn stop(&self) {
        info!("server: sending stop message");
        let _ = self.sender.send(ServerMessages::Shutdown);
    }

    #[allow(dead_code)]
    pub fn get_info(&self, tx: Sender<Commands>) {
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert(String::from("name"), self.name.to_string().clone());
        params.insert(String::from("owner"), self.author.to_string().clone());
        
        let command = Commands::Info(Some(params));
        tx.send(command).unwrap();
    }

    #[allow(dead_code)]
    pub fn update_all_clients(&self, command: Commands){
        let clients = self.connected_clients.lock().unwrap();
        for client in clients.values(){
            client.sender.send(command.clone()).unwrap();
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
        let _ = stream.write(data.to_string().as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

impl ToString for Server {
    fn to_string(&self) -> std::string::String { todo!() }
}

impl Drop for Server {
    fn drop(&mut self) {
        println!("server dropped");
        let _ = self.sender.send(ServerMessages::Shutdown);
    }
}