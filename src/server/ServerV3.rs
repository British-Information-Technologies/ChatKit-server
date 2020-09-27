use std::{sync::{Mutex, Arc}, net::{TcpStream, TcpListener}, collections::HashMap, io, io::{Write, Read}, thread};
use crate::{
    server::client::clientV3::Client,
    commands::Commands
};
use crossbeam_channel::{Sender, Receiver, unbounded};
use log::info;
use std::time::Duration;

#[derive(Debug)]
pub enum ServerMessages {
    RequestUpdate(Arc<Mutex<TcpStream>>),
    RequestInfo(String, Arc<Mutex<TcpStream>>),
    Disconnect(String),
    Shutdown,
}

pub enum ServerState {
    starting,
    started,
    stopping,
    stopped,
}

// MARK: - server struct
pub struct Server {
    pub name: String,
    pub address: String,
    pub owner: String,

    pub state: ServerState,

    connected_clients: HashMap<String, Client>,

    sender: Sender<ServerMessages>,
    receiver: Receiver<ServerMessages>,
    listener: Option<TcpListener>,

    buffer: [u8; 1024],

    client_list_changed_handle: Box<dyn Fn(&Server)>,

    // metrics
    pub o2s_rqst: usize,
    pub c2s_msgs: usize,
    pub s2s_msgs: usize,
    pub s2c_msgs: usize,
}

// MARK: - server implemetation
impl Server {
    pub fn new(name: &str, address: &str, author: &str) -> Result<Self, io::Error> {
        // creating server channels
        let (sender, receiver) = unbounded();

        Ok(
            Self {
                // server data
                name: name.to_string(),
                address: address.to_string(),
                owner: author.to_string(),
                connected_clients: HashMap::new(),
                state: ServerState::ready,

                // messages & connections
                sender,
                receiver,
                listener: None,

                buffer: [0; 1024],

                // event handles
                client_list_changed_handle: Box::new(|_s| info!("Server: client list changed.")),

                // metrics
                o2s_rqst: 0,
                c2s_msgs: 0,
                s2s_msgs: 0,
                s2c_msgs: 0,
            }
        )
    }

    #[allow(dead_code)]
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    #[allow(dead_code)]
    pub fn get_address(&self) -> String {
        self.address.clone()
    }

    #[allow(dead_code)]
    pub fn get_owner(&self) -> String {
        self.owner.clone()
    }

    pub fn tick(&mut self) {

        // check to see if this server is ready to execute things.
        if self.state != ServerState::ready {
            ()
        }

        // check for any server messages in the channel
        println!("server: getting messages");
        for i in self.receiver.try_iter() {
            match i {
                // server calls
                ServerMessages::Shutdown => {
                    self.s2s_msgs += 1;

                    println!("server: shutting down...");

                    for (k, v) in self.connected_clients.iter() {
                        v.sender.send(Commands::Disconnect(None));
                    }
                    self.state = ServerState::stopping;
                },

                // client requests
                ServerMessages::RequestUpdate(stream_arc) => {
                    self.c2s_msgs += 1;

                    for (_k, v) in self.connected_clients.iter() {
                        let mut stream = stream_arc.lock().unwrap();
                        let _ = Server::send_data(&mut stream, v.to_string().as_str());
                        let data = Server::recv_data(&mut stream, &mut self.buffer).unwrap_or(Commands::Error(None));

                        if data == Commands::Success(None) {
                            println!("Success Confirmed");
                        } else {
                            println!("No success read");
                            let error = Commands::Error(None);
                            let _ = Server::send_data(&mut stream, error.to_string().as_str());
                        }
                    }
                },

                // client requests for info
                ServerMessages::RequestInfo(uuid, stream_arc) => {
                    self.c2s_msgs += 1;

                    let mut stream = stream_arc.lock().unwrap();

                    if let Some(client) = self.connected_clients.get(&uuid) {

                        let params: HashMap<String, String> = [
                            (String::from("uuid"), client.get_uuid()),
                            (String::from("name"), client.get_username()),
                            (String::from("host"), client.get_address())
                        ].iter().cloned().collect();

                        let command = Commands::Success(Some(params));
                        let _ = Server::send_data(&mut stream, command.to_string().as_str());

                    } else {
                        let command = Commands::Success(None);
                        let _ = Server::send_data(&mut stream, command.to_string().as_str());
                    }
                },

                // client disconnect requests
                ServerMessages::Disconnect(uuid) => {
                    self.c2s_msgs += 1;

                    self.connected_clients.remove(&uuid.to_string());

                    let params: HashMap<String, String> = [(String::from("uuid"), uuid)].iter().cloned().collect();

                    let command = Commands::ClientRemove(Some(params));
                    let _ = self.connected_clients.iter().map(move |(_k, v)| {v.get_sender().send(command.clone())});

                },
            }
        }

        println!("server: checking for new connections");
        if let Ok((mut stream, _addr)) = self.listener.accept() {
            let _ = stream.set_read_timeout(Some(Duration::from_millis(1000)));
            let _ = stream.set_nonblocking(false);

            let request = Commands::Request(None);
            let _ = Server::send_data(&mut stream, &request.to_string().as_str());

            match Server::recv_data(&mut stream, &mut self.buffer) {


                Ok(Commands::Connect(Some(data))) => {
                    self.o2s_rqst += 1;

                    let uuid = data.get("uuid").unwrap();
                    let username = data.get("name").unwrap();
                    let address = data.get("host").unwrap();

                    info!("{}", format!("Server: new client from {}", address ));

                    let client = Client::new(stream, self.sender.clone(), &uuid, &username, &address);

                    self.connected_clients.insert(uuid.to_string(), client);

                    let params: HashMap<String, String> = [(String::from("name"), username.clone()), (String::from("host"), address.clone()), (String::from("uuid"), uuid.clone())].iter().cloned().collect();
                    let new_client = Commands::Client(Some(params));

                    let _ = self.connected_clients.iter().map( |(_k, v)| v.sender.send(new_client.clone()));
                },


                Ok(Commands::Info(None)) => {
                    self.o2s_rqst += 1;

                    println!("Server: info requested");
                    let params: HashMap<String, String> = [(String::from("name"), self.name.to_string().clone()), (String::from("owner"), self.owner.to_string().clone())].iter().cloned().collect();
                    let command = Commands::Info(Some(params));

                    let _ = Server::send_data(&mut stream, command.to_string().as_str());
                },

                Err(_) => println!("ERROR: stream closed"),

                // TODO: - correct connection reset error when getting info.
                _ => {
                    println!("Server: Invalid command sent");
                    let _ = Server::send_data(&mut stream, Commands::Error(None).to_string().as_str());
                },
            }
        }

        println!("server: handing control to clients");
        for (_k, client) in self.connected_clients.iter_mut() {
            client.handle_connection();
        }
    }

    pub fn start(&mut self) -> Result<(), io::Error> {

        let listener = TcpListener::bind(self.address)?;
        listener.set_nonblocking(true)?;

        self.listener = Some(listener);
    }

    pub fn stop(&mut self) {
        info!("server: sending stop message");
        let _ = self.sender.send(ServerMessages::Shutdown);
        self.state = ServerState::stopping;
    }

    fn send_data(stream: &mut TcpStream, data: &str) -> Result<(), io::Error>{
        println!("Transmitting...");
        println!("data: {}", data);

        /*
         * This will throw an error and crash any thread, including the main thread, if
         * the connection is lost before transmitting. Maybe change to handle any exceptions
         * that may occur.
         */
        let _ = stream.write(data.to_string().as_bytes())?;
        stream.flush()?;
        Ok(())
    }

    fn recv_data(stream: &mut TcpStream, buffer: &mut [u8; 1024]) -> Result<Commands, io::Error> {
        let _ = stream.read(buffer)?;
        let command = Commands::from(buffer);

        Ok(command)
    }
}

impl ToString for Server {
    fn to_string(&self) -> std::string::String { todo!() }
}

impl Drop for Server {
    fn drop(&mut self) {
        println!("server dropped");
        let _ = self.sender.send(ServerMessages::Shutdown);
    }
}
