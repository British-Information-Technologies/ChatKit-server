use std::{io::{Read, Write}, io, net::TcpStream};
use std::time::Duration;

use zeroize::Zeroize;

use crate::{
    commands::Commands,
    server::client::client_profile::Client,
};

pub struct ClientApi {
    socket: TcpStream,
    addr: String,

    pub on_client_add_handle: fn(Client) -> (),
    pub on_client_remove_handle: fn(String) -> (),
}

impl ClientApi {
    pub fn new(addr: &str) -> Result<Self, io::Error> {
        let socket = TcpStream::connect(addr)?;

        let on_add = |_client: Client| {println!("Client_api: Client added {:?}", _client)};
        let on_remove = |_uuid: String| {println!("Client_api: Client removed {}", _uuid)};
        let a = Self {
            socket,
            addr: addr.to_string(),
            on_client_add_handle: on_add,
            on_client_remove_handle: on_remove,
        };
        Ok(a)
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
        let mut stream = TcpStream::connect_timeout(&addr, Duration::from_millis(1000))?;
        
        let _ = stream.read(&mut buffer)?; 
        println!("data recieved: {:?}", &buffer[0..20]);
        match Commands::from(&mut buffer) {
            Commands::Request(None) => {
                println!("zeroing");
                buffer.zeroize();
                println!("writing");
                let sending_command = Commands::Info(None).to_string();
                println!("sending string: {:?} as_bytes: {:?}", &sending_command, &sending_command.as_bytes());
                stream.write_all(sending_command.as_bytes())?;
                stream.flush()?;
                println!("reading");
                let bytes = stream.read(&mut buffer)?;
                println!("new buffer size: {:?} contents: {:?}", bytes, &buffer[0..20]);
                println!("commanding");
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
