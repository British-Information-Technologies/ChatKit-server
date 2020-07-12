mod server;

use crate::server::client::client_profile::Client;
use crate::server::server_profile::Server;
use std::net::{TcpStream, TcpListener};
use rust_chat_server::ThreadPool;
use std::sync::{Arc, Barrier, Mutex};
use std::collections::HashMap;

fn main(){
    let server_name = String::from("Server-01");
    let server_address = String::from("0.0.0.0:6000");
    let server_author = String::from("nope@live.co.uk");
    let connected_clients: Arc<Mutex<HashMap<String,Client>>> = Arc::new(Mutex::new(HashMap::new()));

    let server = Arc::new(Server::new(&server_name, &server_address, &server_author, &connected_clients));
    //server.start();
    let listener = TcpListener::bind(server.get_address()).unwrap();
    let pool = ThreadPool::new(10);
    //stream.set_read_timeout(Some(Duration::from_millis(3000))).unwrap();
    loop{
        if let Ok((mut stream, addr)) = listener.accept(){
            println!("Connected: {}", addr);
            let server = Arc::clone(&server);
            let connected_clients = Arc::clone(&connected_clients);

            pool.execute(move || {
                match server.establish_connection(stream){
                    Ok(mut client) => client.handle_connection(&server, &connected_clients),
                    Err(error) => println!("---connction to client failed---"),
                }
            });
        }
    }
}
