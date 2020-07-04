//mod client_management;
mod server;

use crate::server::client::client_profile::Client;
use crate::server::server_profile::Server;

use std::collections::HashMap;
use std::collections::VecDeque;

fn main(){
    let server_name = String::from("Server-01");
    let server_address = String::from("0.0.0.0:6000");
    let server_author = String::from("nope@live.co.uk");

    let server = Server::new(&server_name, &server_address, &server_author);
    server.start();
}
