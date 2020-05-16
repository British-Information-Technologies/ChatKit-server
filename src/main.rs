mod client_management;

use crate::client_management::client_profile::Client; 
use rust_chat_server::ThreadPool;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::Mutex;
use std::sync::Arc;
//use std::collections::HashMap;
use std::io::prelude::*;

fn main(){
    let listener = TcpListener::bind("127.0.0.1:6001").unwrap();
    let pool = ThreadPool::new(4);

    //let mut connected_clients: Vec<String> = Vec::new();
    //let connected_clients = Arc::new(Mutex::new(Vec::new()));
    //let connected_clients = Arc::new(Mutex::new(HashMap::new()));
    
    //let mut connected_clients = vec![];
    //let mut connected_clients: Vec<String> = vec![];
    let connected_clients = Arc::new(Mutex::new(vec![]));

    loop{
    //for stream in listener.incoming(){
        if let Ok((stream, address)) = listener.accept(){
            println!("Connected: {}", address);
            //let stream = stream.unwrap();
            let clients_ref = Arc::clone(&connected_clients);

            pool.execute(move || {
                handle_connection(stream, clients_ref, address.to_string());
            });
        }
    }

    //println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream, connect_clients: Arc<Mutex<Vec<String>>>, new_address: String){//vec needs to be of type clients
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let incoming_message = String::from_utf8_lossy(&buffer[..]);
    println!("Request: {}", incoming_message);

    let connection_status  = b"STATUS:online";
    let clients_request = b"CLIENT:request_clients";
    let ip_request = b"CLIENT:request_ip";

    if buffer.starts_with(connection_status){
        let mut new_uuid = String::new();
        let mut new_username = String::new();
        //let ip = format!("{}", stream.local_addr().unwrap());
        
        for data in incoming_message.split_whitespace(){
            if data.contains("UUID"){
                for id in data.split(":"){
                    if !id.contains("UUID"){
                        new_uuid.push_str(id);
                    }
                }
            }else if data.contains("UNAME"){
                for uname in data.split(":"){
                    if !uname.contains("UNAME"){
                        new_username.push_str(uname);
                    }
                }
            }
        }
        //let mut clients_hashmap = clients_ref.lock().unwrap();
        //let user_details = [username, ip_request];
        //clients_hashmap.insert(uuid,user_details);
        let client = Client{
            uuid: new_uuid,
            username: new_username,
            address: new_address,
        };
    }
}
    /*else if buffer.starts_with(clients_request){
        handle_clients_request(stream, &incoming_message, clients_ref);
    }else if buffer.starts_with(ip_request){
        handle_ip_request(stream, &incoming_message, clients_ref);
    }

    //stream.write(response.as_bytes()).unwrap();
    //stream.flush().unwrap();
}

fn handle_clients_request(stream: TcpStream, incoming_message: &str, clients_ref: Arc<Mutex<HashMap<String,String>>>){
    let username = identify_requested_username(incoming_message);
    let clients = get_requested_users(&username, clients_ref);
    transmit_data(stream, clients);
}

fn identify_requested_username(incoming_message: &str) -> String{
    let mut username = String::new();
    for data in incoming_message.split_whitespace(){
        if data.contains("UNAME"){
            username = data.to_string();
            break;
        }
    }
    username
}

fn get_requested_users(username: &str, clients_ref: Arc<Mutex<HashMap<String,String>>>) -> HashMap<String,String>{
    let clients_hashmap = clients_ref.lock().unwrap();
    let mut new_clients_hashmap = HashMap::new();

    for (k, v) in clients_hashmap.iter(){
        if v.eq(username){
            new_clients_hashmap.insert(k.to_string(),v.to_string());
        }
    }
    new_clients_hashmap
}

fn transmit_data(mut stream: TcpStream, users: HashMap<String,String>){
    let mut response = String::new();
    
    for (uuid, username) in users.iter(){
        response.push_str(&format!("uuid:{}username:{}\n",uuid,username));
    }

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_ip_request(stream: TcpStream, incoming_message: &str, clients_ref: Arc<Mutex<HashMap<String,String>>>){
    let client_ip = identify_requested_ip(incoming_message);
    let client = get_requested_user(&client_ip, clients_ref);
    transmit_data(stream, client);
}

fn identify_requested_ip(incoming_message: &str) -> String{
}

fn get_requested_user(client_ip: &str, clients_ref: Arc<Mutex<HashMap<String,String>>>) -> HashMap<String,String>{}
*/
