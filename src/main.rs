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
 */

mod client_management;
mod protocols;

extern crate regex;

use crate::client_management::client_profile::Client; 
use crate::protocols::commands::Commands;
use rust_chat_server::ThreadPool;

use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;
use std::io::prelude::*;
use regex::Regex;

fn main(){
    let listener = TcpListener::bind("127.0.0.1:6001").unwrap();
    let pool = ThreadPool::new(10);
    /*
     * Always dedicate 1/4 size of the connected users for updating clients
     */
    let connected_clients = Arc::new(Mutex::new(HashMap::new()));

    loop{
        if let Ok((stream, address)) = listener.accept(){
            println!("Connected: {}", address);
            let clients_ref = Arc::clone(&connected_clients);

            pool.execute(move || {
                handle_connection(stream, &clients_ref, &address.to_string());
            });
        }
    }
}

fn handle_connection(mut stream: TcpStream, clients_ref: &Arc<Mutex<HashMap<String,Client>>>, new_address: &String){//vec needs to be of type clients
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let incoming_message = String::from_utf8_lossy(&buffer[..]);
    let data: Vec<String> = tokenize(&incoming_message);

    println!("Request: {}", incoming_message);
    println!("Data: {:?}", data);

    let command = match_command(&data); 
    command.execute(stream, &data, new_address, clients_ref);
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
        "!client:" => Commands::Client,
        "!test:" => Commands::Test,
        "!message:" => Commands::Message,
        _ => Commands::Unknown,
    }
}
