use std::net::TcpStream;
use std::io::Write;
use std::collections::HashMap;
use regex::Regex;
use crate::server::commands::{Commands, OutboundCommands};
use std::sync::Arc;
use parking_lot::FairMutex;
use std::collections::VecDeque;

pub fn transmit_data(mut stream: &TcpStream, data: &str){
    println!("Transmitting...");
    println!("data: {}",data);

    stream.write(data.to_string().as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn tokenize(incoming_message: &str) -> HashMap<String, String>{
    let mut data: HashMap<String, String> = HashMap::new();

    for mat in Regex::new(r###"(\?|!)([a-zA-z0-9]*):|([a-zA-z]*):([a-zA-Z0-9\-\+\[\]{}_=/]+|("(.*?)")+)"###).unwrap().find_iter(incoming_message){
        if match match_command(&mat.as_str().to_string()) { Commands::Unknown => false, _ => true,} || match match_outbound_command(&mat.as_str().to_string()) { OutboundCommands::Unknown => false, _ => true,}{
            data.insert("command".to_string(), mat.as_str().to_string());
        }else{
            let segment = mat.as_str().to_string();
            let contents: Vec<&str> = segment.split(":").collect();
            println!("key: {}, value: {}", contents[0].to_string(), contents[1].to_string());
            data.insert(contents[0].to_string(), contents[1].to_string());
        }
    }
    data
}

pub fn match_command(command: &String) -> Commands{
    match command.as_str(){
        "!info:" => Commands::Info,
        "!connect:" => Commands::Connect,
        "!disconnect:" => Commands::Disconnect,
        "!clientUpdate:" => Commands::ClientUpdate,
        "!clientInfo:" => Commands::ClientInfo,
        _ => Commands::Unknown,
    }
}

pub fn match_outbound_command(command: &String) -> OutboundCommands{
    match command.as_str(){
        "!client:" => OutboundCommands::Client,
        "!clientRemove:" => OutboundCommands::ClientRemove,
        _ => OutboundCommands::Unknown,
    }
}

pub fn format_data(message_queue: &Arc<FairMutex<VecDeque<String>>>) -> HashMap<String, String>{
    //copy data from queue
    let locked_message_queue = message_queue.lock();
    let message = locked_message_queue.get(0).unwrap();
    println!("msg: {}", message);

    tokenize(&message)
}

pub fn extract_fields(data: &Vec<String>) -> (String, String){
    let mut uuid = String::from("");
    let mut username = String::from("");

    for field in data{
        if field.contains("uuid:"){
            let contents: Vec<&str> = field.split(":").collect();
            uuid.push_str(contents[1]);
        }else if field.contains("username:"){
            let contents: Vec<&str> = field.split(":").collect();
            username.push_str(contents[1]);
        }
    }

    (uuid, username)
}
