mod server;
// mod server_v2;
use crate::server::server_profile::Server;

fn main(){
    let server_name: &'static str = "Server-01";
    let server_address: &'static str = "0.0.0.0:6000";
    let server_author: &'static str = "noreply@email.com";
    /*
    let server_name = String::from("Server-01");
    let server_address = String::from("0.0.0.0:6000");
    let server_owner = String::from("noreply@email.com");
    */

    let server = Server::new(server_name, server_address, server_author);
    //server.start();
}
