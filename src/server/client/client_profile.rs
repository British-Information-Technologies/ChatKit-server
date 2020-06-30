extern crate regex;

use crate::server::commands::Commands;
use crate::server::utility;

use std::collections::VecDeque;
use std::net::{Shutdown, TcpStream};
use std::sync::{Arc, Barrier, Mutex};
use std::rc::Rc;
use crossbeam_channel::{Receiver, TryRecvError};
use parking_lot::FairMutex;
use std::collections::HashMap;
use dashmap::DashMap;
use std::io::prelude::*;
use std::time::Duration;

#[derive(Clone)]
pub struct Client{
    connected: bool,
    stream: Arc<TcpStream>,
    uuid: String,
    username: String,
    address: String,
}

impl Client{
    pub fn new(stream: Arc<TcpStream>, uuid: &String, username: &String, address: &String) -> Client{
        Client{
            connected: true,
            stream: stream,
            uuid: uuid.to_string(),
            username: username.to_string(),
            address: address.to_string(),
        }
    }

    pub fn get_stream(&self) -> &TcpStream{
        &self.stream
    }
    
    pub fn get_uuid(&self) -> &String{
        &self.uuid
    }

    pub fn get_username(&self) -> &String{
        &self.username
    }

    pub fn get_address(&self) -> &String{
        &self.address
    }

    pub fn disconnect(&mut self){
        self.stream.shutdown(Shutdown::Both).expect("shutdown call failed");
        self.connected = false;
    }
  
    pub fn handle_connection(&mut self, clients_ref: &Arc<Mutex<HashMap<String, Client>>>, message_queue: &Arc<FairMutex<VecDeque<String>>>, client_rx: Receiver<Arc<Barrier>>){
        //self.stream.set_read_timeout(Some(Duration::from_millis(3000))).unwrap();
        let mut buffer = [0; 1024];       
        
        while self.connected {
            if !message_queue.lock().is_empty() {
                match client_rx.try_recv(){
                    /*handle our data*/
                    Ok(sync_group) => {
                        println!("data present");
                        let data = utility::format_data(message_queue);
                        let command = utility::match_outbound_command(&data.get("command").unwrap());
                        println!("waiting 1");
                        sync_group.wait();
                        println!("executing");
                        command.execute(&self, &mut buffer, &data);
                        println!("waiting 2");
                        sync_group.wait();
                        println!("client updated");
                    },
                    /*sender disconnected*/
                    Err(TryRecvError::Disconnected) => {},
                    /*no data available yet*/
                    Err(TryRecvError::Empty) => {},
                }
            }
            
            match self.stream.peek(&mut buffer){
                Ok(_) => {
                    //self.stream.lock().unwrap().read(&mut buffer).unwrap();
                    self.get_stream().read(&mut buffer).unwrap();
                    
                    let incoming_message = String::from_utf8_lossy(&buffer[..]);
                    //let data: Vec<String> = utility::tokenize(&incoming_message);
                    let data: HashMap<String, String> = utility::tokenize(&incoming_message);

                    println!("Request: {}", incoming_message);
                    //println!("Data: {:?}", data);

                    //let command = utility::match_command(&data[0]);
                    let command = utility::match_command(&data.get("command").unwrap());
                    
                    if match command{ Commands::Connect => true, _ => false,}{
                        println!("Error!");
                    } else {
                        println!("command executing...");
                        command.execute(self, &mut buffer, &data, clients_ref, message_queue);
                    }
                },
                Err(_) => {
                    println!("no data peeked");
                },
            }
        }
        println!("---thread exit---");
    }
}
