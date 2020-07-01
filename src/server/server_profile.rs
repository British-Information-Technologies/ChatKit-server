extern crate regex;

use crate::server::client::client_profile::Client;
use crate::server::commands::Commands;
use crate::server::utility;
use rust_chat_server::ThreadPool;

//use crate::client_management::client_profile::Client; 
//use crate::server::commands::Commands;
//use crate::server::commands::network;

use std::collections::VecDeque;
use std::net::TcpListener;
use std::sync::{Arc, Barrier, Mutex};
use std::rc::Rc;
use crossbeam_channel::{unbounded, Sender, Receiver};
use parking_lot::FairMutex;
use std::collections::HashMap;
use dashmap::DashMap;
use std::io::prelude::*;
use std::thread;

pub struct Server{
    name: String,
    address: String,
    author: String,
    connected_clients: Arc<Mutex<HashMap<String,Client>>>,
    message_queue: Arc<FairMutex<VecDeque<String>>>,
}

impl Server{
    pub fn new(name: &String, address: &String, author: &String) -> Server{
        let connected_clients: Arc<Mutex<HashMap<String,Client>>> = Arc::new(Mutex::new(HashMap::new()));
        let message_queue: Arc<FairMutex<VecDeque<String>>> = Arc::new(FairMutex::new(VecDeque::new()));
        
        Server{
            name: name.to_string(),
            address: address.to_string(),
            author: author.to_string(),
            connected_clients: connected_clients,
            message_queue: message_queue,
        }
    }

    pub fn start(&self){
        let listener = TcpListener::bind(self.address.clone()).unwrap();
        let pool = ThreadPool::new(10);
        //let connected_clients = Arc::new(Mutex::new(self.connected_clients.clone()));
        //let message_queue: Arc<FairMutex<VecDeque<String>>> = Arc::new(FairMutex::new(self.message_queue.clone()));
    
        let (tx,rx): (Sender<Arc<Barrier>>, Receiver<Arc<Barrier>>) = unbounded();
        let (clock_tx, _) = (tx.clone(), rx.clone());

        thread::spawn({
            let connected_clients = Arc::clone(&self.connected_clients);
            let message_queue = Arc::clone(&self.message_queue);
            move || {
                loop{
                    let online_clients = connected_clients.lock().unwrap().len();
                    if !message_queue.lock().is_empty(){
                        println!("message on queue detected");
                        let sync_group = Arc::new(Barrier::new(online_clients+1));
                        println!("sending to threads... {}",online_clients);
                        for _ in 0..online_clients{
                            println!("thread");
                            clock_tx.send(sync_group.clone()).unwrap();
                        }
                        println!("all threads updated!");
                        sync_group.wait();
                        println!("data removed");
                        message_queue.lock().pop_front();
                        sync_group.wait();
                        println!("clock finished!");
                    }
                }
            }
        });

        //stream.set_read_timeout(Some(Duration::from_millis(3000))).unwrap();
        loop{
            if let Ok((mut stream, addr)) = listener.accept(){
                println!("Connected: {}", addr);

                let connected_clients_ref = Arc::clone(&self.connected_clients);
                let message_queue_ref = Arc::clone(&self.message_queue);
                let (_ , client_rx) = (tx.clone(), rx.clone());

                pool.execute(move || {
                    let mut buffer = [0; 1024];
                    let request = String::from("?request:");
                    utility::transmit_data(&stream, &request);
                    
                    stream.read(&mut buffer).unwrap();
                    let stream = Arc::new(stream);

                    let incoming_message = String::from_utf8_lossy(&buffer[..]);
                    let data: HashMap<String, String> = utility::tokenize(&incoming_message);
                    let command = utility::match_command(&data.get("command").unwrap());
                    
                    if match command{ Commands::Connect => true, _ => false,}{
                        /*
                         * Change so that command is paassed in and then matches how to break the
                         * data up
                         */
                        //let (uuid, username) = utility::extract_fields(&data);
                        let uuid = data.get("uuid").unwrap();
                        let username = data.get("username").unwrap();
                        let address = data.get("host").unwrap();

                        let mut client = Client::new(stream, &uuid, &username, &address);
                        
                        command.execute(&mut client, &mut buffer, &data, &connected_clients_ref, &message_queue_ref);
                        
                        client.handle_connection(&connected_clients_ref, &message_queue_ref, client_rx);
                        //process_connection(&stream, &clients_ref, &message_ref, &address.to_string(), client_rx);
                    }else{
                        //error
                    }
                });
            }
        }
    }
    
}
