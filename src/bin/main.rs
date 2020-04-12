use rust_chat_server::ThreadPool;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;
use std::io::prelude::*;

fn main(){
    let listener = TcpListener::bind("127.0.0.1:6001").unwrap();
    let pool = ThreadPool::new(4);

    //let mut connected_clients: Vec<String> = Vec::new();
    //let connected_clients = Arc::new(Mutex::new(Vec::new()));
    let connected_clients = Arc::new(Mutex::new(HashMap::new()));

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        let clients_ref = Arc::clone(&connected_clients);

        pool.execute(move || {
            handle_connection(stream, clients_ref);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream, clients_ref: Arc<Mutex<HashMap<String,String>>>){
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let incoming_message = String::from_utf8_lossy(&buffer[..]);
    println!("Request: {}", incoming_message);

    let connection_status  = b"STATUS:online";
    let request_client = b"CLIENT:request";
    let mut uuid = String::new();
    let mut username = String::new();

    if buffer.starts_with(connection_status){
        for data in incoming_message.split_whitespace(){
            if data.contains("UUID"){
                for id in data.split(":"){
                    if !id.contains("UUID"){
                        uuid.push_str(id);
                    }
                }
            }else if data.contains("UNAME"){
                for uname in data.split(":"){
                    if !uname.contains("UNAME"){
                        username.push_str(uname);
                    }
                }
            }
        }
        let mut clients_hashmap = clients_ref.lock().unwrap();
        clients_hashmap.insert(uuid,username);
    }else if buffer.starts_with(request_client){

    }

    //stream.write(response.as_bytes()).unwrap();
    //stream.flush().unwrap();
}

fn handle_client_request(){
    identify_requested_username();
    get_requested_username();
}

fn identify_requested_username(){}

fn get_requested_users(){}
