// use std::time::Duration;
// use std::{
//   collections::HashMap,
//   io,
//   io::{Read, Write},
//   net::{TcpListener, TcpStream},
//   sync::{Arc, Mutex},
// };

// use crossbeam_channel::{unbounded, Receiver, SendError, Sender};
// use log::info;

// use crate::commands::Commands;
// use super::client_management;

// #[derive(Debug)]
// pub enum ServerMessages {
//   RequestUpdate(Arc<Mutex<TcpStream>>),
//   RequestInfo(String, Arc<Mutex<TcpStream>>),
//   Disconnect(String),
//   Shutdown,
// }

// pub enum ServerEvent {
//   Stopped,
//   Started,
//   addedClient(Arc<Mutex<Client>>),
// }

// #[allow(dead_code)]
// #[derive(Eq, PartialEq, Debug)]
// pub enum ServerState {
//   Starting,
//   Started,
//   Stopping,
//   Stopped,
// }

// // MARK: - server struct
// #[allow(dead_code)]
// pub struct Server<T> {
//   pub config: ,

//   pub state: ServerState,

//   // to be seperated into a different struct
//   connected_clients: HashMap<String, Client>,

//   server_event_sink: Sender<ServerEvent>,
//   server_message_source: Receiver<T>,

//   message_source_handler: fn(&Self, event: T) -> (),

//   buffer: [u8; 1024],

//   // metrics
//   pub o2s_rqst: usize,
//   pub c2s_msgs: usize,
//   pub s2s_msgs: usize,
//   pub s2c_msgs: usize,
// }

// // MARK: - server implemetation
// impl Server {
//   pub fn new(name: &str, address: &str, author: &str) -> Result<Self, io::Error> {
//     // creating server channels
//     let (sender, receiver) = unbounded();

//     Ok(Self {
//       // server data
//       name: name.to_string(),
//       address: address.to_string(),
//       owner: author.to_string(),
//       connected_clients: HashMap::new(),
//       state: ServerState::Stopped,

//       // messages & connections
//       sender,
//       receiver,
//       listener: None,

//       buffer: [0; 1024],

//       // metrics
//       o2s_rqst: 0,
//       c2s_msgs: 0,
//       s2s_msgs: 0,
//       s2c_msgs: 0,
//     })
//   }

//   pub fn get_name(&self) -> String {
//     self.name.clone()
//   }

//   pub fn get_address(&self) -> String {
//     self.address.clone()
//   }

//   pub fn get_owner(&self) -> String {
//     self.owner.clone()
//   }

//   fn handle_server_messages(&mut self) -> Result<(), Vec<Result<(), ServerError>>> {
//     // check for any server messages in the channel
//     println!("server: getting messages");
//     self.receiver.try_iter().map(|msg| {
//       let _ = match msg {
//         // request the server to shutdown
//         // TODO: - move this into the stop method
//         ServerMessages::Shutdown => {
//           println!("server: shutting down...");

//           let results = self
//             .connected_clients
//             .iter()
//             .map(|(_k, v)| v.sender.send(Commands::Disconnect(None)))
//             .cloned()
//             .collect();

//           self.state = ServerState::Stopping;
//         }

//         // a client requests an updated list of clients
//         ServerMessages::RequestUpdate(stream_arc) => {
//           self.c2s_msgs += 1;

//           self.connected_clients.iter().map(|(_k, v)| {
//             let mut stream = stream_arc.lock().unwrap();
//             let _ = Server::send_data(&mut stream, v.to_string().as_str());
//             let data =
//               Server::recv_data(&mut stream, &mut self.buffer).unwrap_or(Commands::Error(None));

//             if data == Commands::Success(None) {
//               println!("Success Confirmed");
//             } else {
//               println!("No success read");
//               let error = Commands::Error(None);
//               let _ = Server::send_data(&mut stream, error.to_string().as_str());
//             }
//           })
//         }

//         // a client requests for the servers info
//         ServerMessages::RequestInfo(uuid, stream_arc) => {
//           self.c2s_msgs += 1;

//           let mut stream = stream_arc.lock().unwrap();

//           if let Some(client) = self.connected_clients.get(&uuid) {
//             let params: HashMap<String, String> = [
//               (String::from("uuid"), client.get_uuid()),
//               (String::from("name"), client.get_username()),
//               (String::from("host"), client.get_address()),
//             ]
//             .iter()
//             .cloned()
//             .collect();

//             let command = Commands::Success(Some(params));
//             let _ = Server::send_data(&mut stream, command.to_string().as_str());
//           } else {
//             let command = Commands::Success(None);
//             let _ = Server::send_data(&mut stream, command.to_string().as_str());
//           }
//         }

//         // a client requests to disconnect
//         ServerMessages::Disconnect(uuid) => {
//           self.c2s_msgs += 1;

//           self.connected_clients.remove(&uuid.to_string());

//           let params: HashMap<String, String> =
//             [(String::from("uuid"), uuid)].iter().cloned().collect();

//           let command = Commands::ClientRemove(Some(params));
//           let _ = self
//             .connected_clients
//             .iter()
//             .map(move |(_k, v)| v.get_sender().send(command.clone()));
//         }
//       };
//     });
//     Ok(())
//   }

//   #[allow(dead_code)]
//   pub fn tick(&mut self) -> Result<(), ServerError> {
//     // check to see if this server is ready to execute things.
//     if self.state == ServerState::Stopped {
//       Err(ServerIsStopped)
//     }

//     self.handle_server_messages();

//     println!("server: checking for new connections");
//     if let Ok((mut stream, _addr)) = self
//       .listener
//       .as_ref()
//       .expect("tcpListener not here")
//       .accept()
//     {
//       let _ = stream.set_read_timeout(Some(Duration::from_millis(1000)));
//       let _ = stream.set_nonblocking(false);

//       let request = Commands::Request(None);
//       let _ = Server::send_data(&mut stream, &request.to_string().as_str());

//       match Server::recv_data(&mut stream, &mut self.buffer) {
//         Ok(Commands::Connect(Some(data))) => {
//           self.o2s_rqst += 1;

//           let uuid = data.get("uuid").unwrap();
//           let username = data.get("name").unwrap();
//           let address = data.get("host").unwrap();

//           info!("{}", format!("Server: new client from {}", address));

//           let client = Client::new(stream, self.sender.clone(), &uuid, &username, &address);

//           self.connected_clients.insert(uuid.to_string(), client);

//           let params: HashMap<String, String> = [
//             (String::from("name"), username.clone()),
//             (String::from("host"), address.clone()),
//             (String::from("uuid"), uuid.clone()),
//           ]
//           .iter()
//           .cloned()
//           .collect();
//           let new_client = Commands::Client(Some(params));

//           let _ = self
//             .connected_clients
//             .iter()
//             .map(|(_k, v)| v.sender.send(new_client.clone()));
//         }

//         Ok(Commands::Info(None)) => {
//           self.o2s_rqst += 1;

//           println!("Server: info requested");
//           let params: HashMap<String, String> = [
//             (String::from("name"), self.name.to_string().clone()),
//             (String::from("owner"), self.owner.to_string().clone()),
//           ]
//           .iter()
//           .cloned()
//           .collect();
//           let command = Commands::Info(Some(params));

//           let _ = Server::send_data(&mut stream, command.to_string().as_str());
//         }

//         Err(_) => println!("ERROR: stream closed"),

//         // TODO: - correct connection reset error when getting info.
//         _ => {
//           println!("Server: Invalid command sent");
//           let _ = Server::send_data(&mut stream, Commands::Error(None).to_string().as_str());
//         }
//       }
//     }

//     println!("server: handing control to clients");
//     for (_k, client) in self.connected_clients.iter_mut() {
//       client.handle_connection();
//     }

//     Ok(())
//   }

//   #[allow(dead_code)]
//   pub fn start(&mut self) -> Result<(), io::Error> {
//     let listener = TcpListener::bind(&self.address)?;
//     listener.set_nonblocking(true)?;

//     self.listener = Some(listener);
//     self.state = ServerState::Started;

//     Ok(())
//   }

//   #[allow(dead_code)]
//   pub fn stop(&mut self) -> Result<(), SendError<ServerMessages>> {
//     info!("server: sending stop message");
//     self.sender.send(ServerMessages::Shutdown)?;
//     self.state = ServerState::Stopping;
//     Ok(())
//   }

//   #[allow(dead_code)]
//   fn send_data(stream: &mut TcpStream, data: &str) -> Result<(), io::Error> {
//     println!("Transmitting...");
//     println!("data: {}", data);

//     /*
//      * This will throw an error and crash any thread, including the main thread, if
//      * the connection is lost before transmitting. Maybe change to handle any exceptions
//      * that may occur.
//      */
//     let _ = stream.write(data.to_string().as_bytes())?;
//     stream.flush()?;
//     Ok(())
//   }

//   #[allow(dead_code)]
//   fn recv_data(stream: &mut TcpStream, buffer: &mut [u8; 1024]) -> Result<Commands, io::Error> {
//     let _ = stream.read(buffer)?;
//     let command = Commands::from(buffer);

//     Ok(command)
//   }
// }

// impl Drop for Server {
//   // TODO: - implement the drop logic
//   // this includes signaling all clients to disconnect
//   fn drop(&mut self) {}
// }

// #[cfg(test)]
// mod server_v3_tests {
//   use crate::server::server_v3::{Server, ServerState};

//   #[test]
//   fn test_creation_and_drop() {
//     let server =
//       Server::new("test server", "0.0.0.0:6000", "michael").expect("server creation failed");

//     assert_eq!(server.name, "test server");
//     assert_eq!(server.address, "0.0.0.0:6000");
//     assert_eq!(server.owner, "michael");
//   }

//   #[test]
//   fn test_server_start() {
//     let mut server =
//       Server::new("test server", "0.0.0.0:6000", "michael").expect("server creation failed");

//     let result = server.start();

//     assert!(result.is_ok());
//     assert_eq!(server.state, ServerState::Started);
//   }

//   #[test]
//   fn test_server_stop() {
//     let mut server =
//       Server::new("test server", "0.0.0.0:6000", "michael").expect("server creation failed");

//     let _ = server.start();
//     let result = server.stop();

//     assert!(result.is_ok());
//     assert_eq!(server.state, ServerState::Stopping);
//   }

//   #[test]
//   fn test_server_start_stop_and_one_tick() {
//     let mut server =
//       Server::new("test server", "0.0.0.0:6000", "michael").expect("server creation failed");

//     let _ = server.start();
//     let result = server.stop();
//     server.tick();

//     assert!(result.is_ok());
//     assert_eq!(server.state, ServerState::Stopped);
//   }
// }
