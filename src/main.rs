/*
 * Add execute method to Commands enum and implement a struct of CommandInfo into its constructor
 * Change parameters passed into required funtions
 *
 *
 * client sends message to other client, reciever sends confirm message back to sender so sender
 * knows theyre online and accepting packets. If no comfirm comes back to sender, try 2 more times.
 * If no success, request ip from server to double check if theyre online, if ip match, assume
 * theyre offline. If no response from server assume some error has occured or sender is offline.
 * Save messages to be sent and check every few mins to see if they are online.
 *
 */
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
use std::sync::mpsc;
use std::sync::Mutex;
use std::sync::Arc;
use crossbeam_queue::{SegQueue, PushError, PopError};
use crossbeam_utils::sync::WaitGroup;
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
    // Using an ArrayQueue is much faster, but has limited capacity
    //let message_queue = Arc::new(SegQueue::new());
    let message_queue = Arc::new(FairMutex::new(VecDeque::new()));
    let sync_wg = Arc::new(WaitGroup::new());

    thread::spawn({
        let message_ref = Arc::clone(&message_queue);
        move || {
            loop{
                println!("Phase 1");
                //sync_wg.clone().wait();
                println!("Removing item...");
                message_ref.lock().pop_front();

                println!("Phase 2");
                //sync_wg.clone().wait();
                println!("Done");
            }
        }
    });

    loop{
        if let Ok((stream, address)) = listener.accept(){
            println!("Connected: {}", address);
            let clients_ref = Arc::clone(&connected_clients);
            let message_ref = Arc::clone(&message_queue);
            //let wg = sync_wg.clone();
            
            pool.execute(move || {
                handle_connection(&stream, &clients_ref, &message_ref, &sync_wg, &address.to_string());
            });
        }
    }
}

fn handle_connection(mut stream: &TcpStream, clients_ref: &Arc<Mutex<HashMap<String,Client>>>, message_queue: &Arc<FairMutex<VecDeque<String>>>, sync_wg: &WaitGroup, new_address: &String){
    let wg = sync_wg.clone();
    stream.set_read_timeout(Some(Duration::from_millis(3000))).unwrap();
    let mut buffer = [0; 1024];
    
    let request = String::from("?request:");
    network::transmit_data(stream, &request);

    /*
     * Share a message queue with all threads where operations that need to be performed are added
     * to the queue, each thread will then increment a integer till its the same as the amount of
     * clients online, then all threads will execute the command together.
     */
    let mut connected = false;
   
    // do while loop
    while {
        println!("loop start");
        if connected == true && !message_queue.lock().is_empty() {
            let (command, data) = format_data(message_queue);
            let command = match_outbound_command(&command);

            //BUG: gets stuck at waiting...

            //change to a wait group so all threads are in sync
            println!("waiting...");
            //sync_wg.clone().wait();
            sync_wg.wait();
            println!("done");

            //execute copied command
            command.execute(stream, &mut buffer, &data);
            println!("waiting...");
            //sync_wg.clone().wait();
            println!("done");
            //remove data from front of queue
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
        "?request:" => Commands::Request,
        "?info!" => Commands::Info,
        "!success:" => Commands::Success,
        "!error:" => Commands::Error,
        "!connect:" => Commands::Connect,
        "!disconnect:" => Commands::Disconnect,
        "!clientUpdate:" => Commands::ClientUpdate,
        "!clientInfo:" => Commands::ClientInfo,
        "!client:" => Commands::Client,
        "!test:" => Commands::Test,
        "!message:" => Commands::Message,
        _ => Commands::Unknown,
    }
}

fn match_outbound_command(data: &Vec<String>) -> OutboundCommands{
    match data[0].as_str(){
        "!clientUpdate:" => OutboundCommands::ClientUpdate,
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
