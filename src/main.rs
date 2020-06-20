mod client_management;
mod protocols;

extern crate regex;

use crate::client_management::client_profile::Client; 
use crate::protocols::commands::Commands;
use crate::protocols::commands::OutboundCommands;
use crate::protocols::commands::network;
use rust_chat_server::ThreadPool;

use std::collections::VecDeque;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::{Arc, Barrier, Mutex};
use crossbeam_channel::{unbounded, Sender, Receiver, TryRecvError};
use parking_lot::FairMutex;
use std::collections::HashMap;
use dashmap::DashMap;
use std::io::prelude::*;
use std::time::Duration;
use regex::Regex;
use std::thread;

fn main(){
    let listener = TcpListener::bind("127.0.0.1:6001").unwrap();
    let pool = ThreadPool::new(10);
    let connected_clients = Arc::new(Mutex::new(HashMap::new()));
    let message_queue: Arc<FairMutex<VecDeque<String>>> = Arc::new(FairMutex::new(VecDeque::new()));

    let (tx,rx): (Sender<Arc<Barrier>>, Receiver<Arc<Barrier>>) = unbounded();
    let (clock_tx, _) = (tx.clone(), rx.clone());

    thread::spawn({
        let connected_clients = Arc::clone(&connected_clients);
        let message_queue = Arc::clone(&message_queue);
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
                }
            }
        }
    });

    loop{
        if let Ok((stream, address)) = listener.accept(){
            println!("Connected: {}", address);
            let clients_ref = Arc::clone(&connected_clients);
            let message_ref = Arc::clone(&message_queue);
            let (_ , client_rx) = (tx.clone(), rx.clone());
            
            pool.execute(move || {
                handle_connection(&stream, &clients_ref, &message_ref, &address.to_string(), client_rx);
            });
        }
    }
}

fn handle_connection(mut stream: &TcpStream, clients_ref: &Arc<Mutex<HashMap<String,Client>>>, message_queue: &Arc<FairMutex<VecDeque<String>>>, new_address: &String, client_rx: Receiver<Arc<Barrier>>){
    //let wg = sync_wg.clone();
    stream.set_read_timeout(Some(Duration::from_millis(3000))).unwrap();
    let mut buffer = [0; 1024];
    
    let request = String::from("?request:");
    network::transmit_data(stream, &request);

    let mut connected = false;
   
    // do while loop
    while {
        if connected == true && !message_queue.lock().is_empty() {
            match client_rx.try_recv(){
                /*handle our data*/
                Ok(sync_group) => {
                    println!("data present");
                    let (command, data) = format_data(message_queue);
                    let command = match_outbound_command(&command);
                    println!("waiting 1");
                    sync_group.wait();
                    println!("executing");
                    command.execute(stream, &mut buffer, &data);
                    println!("waiting 2");
                    sync_group.wait();
                    println!("client updated");
                },
                /*sender disconnected*/
                Err(TryRecvError::Disconnected) => {},
                /*no data available yet*/
                Err(TryRecvError::Empty) => {},
            }
        }else{
            stream.read(&mut buffer).unwrap();
            // after timeout handle error and do not execute the code below if there is an error

            let incoming_message = String::from_utf8_lossy(&buffer[..]);
            let data: Vec<String> = tokenize(&incoming_message);

            println!("Request: {}", incoming_message);
            println!("Data: {:?}", data);

            let command = match_command(&data);

            if connected == false && match command{ Commands::Connect => true, _ => false,}{
                command.execute(stream, &mut buffer, &data, new_address, clients_ref, message_queue);
                connected = true;
            }else if connected == true{
                command.execute(stream, &mut buffer, &data, new_address, clients_ref, message_queue);
            }else{
                println!("Error!");
            }
        }
        connected 
    }{}
    println!("---thread exit---");
}

fn tokenize(incoming_message: &str) -> Vec<String>{
    let mut data: Vec<String> = Vec::new();
    for mat in Regex::new(r###"(\?|!)([a-zA-z0-9]*):|([a-zA-z]*):([a-zA-Z0-9\-\+\[\]{}_=/]+|("(.*?)")+)"###).unwrap().find_iter(incoming_message){
        data.push(mat.as_str().to_string());
    }
    data
}

fn match_command(data: &Vec<String>) -> Commands{
    match data[0].as_str(){
        "?info!" => Commands::Info,
        "!connect:" => Commands::Connect,
        "!disconnect:" => Commands::Disconnect,
        "!clientUpdate:" => Commands::ClientUpdate,
        "!clientInfo:" => Commands::ClientInfo,
        _ => Commands::Unknown,
    }
}

fn match_outbound_command(data: &Vec<String>) -> OutboundCommands{
    match data[0].as_str(){
        "!client:" => OutboundCommands::Client,
        "!clientRemove:" => OutboundCommands::ClientRemove,
        _ => OutboundCommands::Unknown,
    }
}

fn format_data(message_queue: &Arc<FairMutex<VecDeque<String>>>) -> (Vec<String>, String){
    //copy data from queue
    let locked_message_queue = message_queue.lock();
    let data = locked_message_queue.get(0).unwrap();
    
    //format the data into a command
    (tokenize(&data), data.clone())
}
