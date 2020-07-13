mod server;
// mod server_v2;

use crate::server::client::client_profile::Client;
use crate::server::server_profile::Server;
use std::net::{TcpStream, TcpListener};
use rust_chat_server::ThreadPool;
use std::sync::{Arc, Barrier, Mutex};
use std::collections::HashMap;

fn main(){
    let server_name = String::from("Server-01");
    let server_address = String::from("0.0.0.0:6000");
    let server_owner = String::from("noreply@email.com");

    let server = Server::new(&server_name, &server_address, &server_owner);
    server.start();
}
