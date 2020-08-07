use std::{net::TcpStream, io::{Write, Read}, io};
use crate::{
    server::client::client_profile::Client,
    commands::Commands,
};
use std::time::Duration;
use std::str::FromStr;
use std::net::SocketAddr;
use zeroize::Zeroize;


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

    pub fn set_on_client_add(&mut self, func: fn(Client) -> ()) {
        self.on_client_add_handle = func;
    }

    pub fn set_on_client_removed(&mut self, func: fn(String) -> ()) {
        self.on_client_remove_handle = func;
    }

    pub fn get_info(host: &str) -> Result<Commands, io::Error> {
        let mut buffer: [u8; 1024] = [0; 1024];
        let addr = host.parse().unwrap();
        let mut stream = TcpStream::connect_timeout(&addr, Duration::from_millis(10000))?;

        stream.read(&mut buffer)?;

        match Commands::from(&buffer) {
            Commands::Request(None) => {
                buffer.zeroize();
                stream.write_all(Commands::Info(None).to_string().as_bytes()).unwrap();
                let a = stream.read(&mut buffer);
                a?;
                Ok(Commands::from(String::from(String::from_utf8_lossy(&buffer))))
            },
            _ => {
                Err(io::Error::new(io::ErrorKind::InvalidData, "the data was not expected"))
            }
        }
    }

    pub fn get_clients(&self) {

    }
}