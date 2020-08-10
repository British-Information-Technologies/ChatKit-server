use std::{
    net::TcpStream,
    io::{Write, Read}
};
use crate::{
    server::client::client_profile::Client,
    commands::Commands,
};
use zeroize::Zeroize;
use std::time::Duration;
use async_std::net::SocketAddrV4;
use std::str::FromStr;
use std::net::SocketAddr;


pub struct ClientApi {
    socket: TcpStream,
    addr: String,

    pub on_client_add_handle: fn(Client) -> (),
    pub on_client_remove_handle: fn(String) -> (),
}

impl ClientApi {
    pub fn new(addr: &str) -> Self {
        let socket = TcpStream::connect(addr).expect("connection failed");

        let on_add = |_client: Client| {println!("Client_api: Client added {:?}", _client)};
        let on_remove = |_uuid: String| {println!("Client_api: Client removed {}", _uuid)};


        Self {
            socket,
            addr: addr.to_string(),
            on_client_add_handle: on_add,
            on_client_remove_handle: on_remove,
        }
    }

    pub fn set_on_client_add(&mut self, Fn: fn(Client) -> ()) {
        self.on_client_add_handle = Fn;
    }

    pub fn set_on_client_removed(&mut self, Fn: fn(String) -> ()) {
        self.on_client_remove_handle = Fn;
    }

    pub fn get_info(host: &str) -> Option<Commands> {
        let mut buffer: [u8; 1024] = [0; 1024];
        let addr = SocketAddr::from_str(host).ok()?;
        let mut stream = TcpStream::connect_timeout(&addr, Duration::from_millis(500)).ok()?;

        stream.read(&mut buffer).ok()?;

        match Commands::from(&mut buffer) {
            Commands::Request(None) => {
                stream.write_all(Commands::Info(None).to_string().as_bytes()).unwrap();
                stream.read(&mut buffer).ok()?;
                Some(Commands::from(String::from(String::from_utf8_lossy(&buffer))))
            },
            _ => {
                None
            }
        }
    }

    pub fn get_clients(&self) {

    }
}
