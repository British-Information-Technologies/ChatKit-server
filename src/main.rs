#[macro_use]
extern crate lazy_static;

mod server;

use crate::server::server_profile::Server;

fn main(){ 
    lazy_static!{
        static ref server_name: &'static str = "Server-01";
        static ref server_address: &'static str = "0.0.0.0:6000";
        static ref server_author: &'static str = "noreply@email.com";
        static ref SERVER: Server<'static> = Server::new(&server_name, &server_address, &server_author);
    }
    /*
    let server_name = String::from("Server-01");
    let server_address = String::from("0.0.0.0:6000");
    let server_author = String::from("noreply@email.com");
    */

    //let server = Server::new(server_name, server_address, server_author);
    SERVER.start();
}
