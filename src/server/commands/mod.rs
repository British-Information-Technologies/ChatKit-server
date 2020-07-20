mod request;
mod info;
mod success;
mod error;
mod connect;
mod disconnect;
mod client_update;
mod client_info;
mod client;
mod test;
mod message;

use std::string::ToString;
use std::collections::HashMap;
use dashmap::DashMap;
use std::borrow::Borrow;
use regex::Regex;
use std::ops::Index;



/*
impl ClientCommands{
    pub fn execute(&self, client: &mut Client, server: &Server, buffer: &mut [u8; 1024], connected_clients: &Arc<Mutex<HashMap<String, Client>>>){
        let stream = client.get_stream();
        match &*self{
            ClientCommands::Info => {
                let server_details = server.get_info();

                client.transmit_success(&server_details);
            },
            ClientCommands::Connect(data) => {
                connect::add_client(connected_clients, client);

                let new_client = ServerCommands::Client(data.clone());
                server.update_all_clients(&new_client);
                
                client.transmit_success(&String::from(""));
            },
            ClientCommands::Disconnect => {
                disconnect::remove_client(connected_clients, client);

                let mut data: HashMap<String, String> = HashMap::new();
                data.insert("uuid".to_string(), client.get_uuid().to_string());

                let old_client = ServerCommands::ClientRemove(data);
                server.update_all_clients(&old_client);

                client.transmit_success(&String::from(""));
                client.disconnect();
                println!("disconnected!");
            },
            ClientCommands::ClientUpdate => {
                let clients_hashmap = connected_clients.lock().unwrap();
                for (key, value) in clients_hashmap.iter(){
                    let formatted_data = client_update::format_client_data(&key, &value);
                    client.transmit_data(&formatted_data);

                    client.confirm_success(buffer, &formatted_data);
                }
                client.transmit_success(&String::from(""));
                client.confirm_success(buffer, &String::from("!success:"));
            },
            ClientCommands::ClientInfo(data) => {
                let requested_data = client_info::get_client_data(connected_clients, &data);
                client.transmit_data(&requested_data);
            },
            ClientCommands::Unknown => {
                println!("Unknown Command");
            },
        }
    }
}

impl ServerCommands{
    pub fn execute(&self, client: &mut Client, buffer: &mut [u8; 1024]){
        match &*self{
            ServerCommands::Client(data) => {
                let mut message = String::from("");
                message.push_str(&"!client: name:");
                message.push_str(&data.get("name").unwrap());
                message.push_str(&" host:");
                message.push_str(&data.get("host").unwrap());
                message.push_str(&" uuid:");
                message.push_str(&data.get("uuid").unwrap());

                client.transmit_data(&message);

                client.confirm_success(buffer, &message);
            },
            ServerCommands::ClientRemove(data) => {
                let mut message = String::from("");
                message.push_str(&"!client: uuid:");
                message.push_str(&data.get("uuid").unwrap());

                client.transmit_data(&message);

                client.confirm_success(buffer, &message);
            },
            ServerCommands::Unknown => {
                println!("Unknown Command!");
            },
        }
    }
}
*/

// MARK: - commands_v2 electric boogaloo 
#[derive(Clone)]
pub enum Commands {
    Request(Option<HashMap<String, String>>),
    Info(Option<HashMap<String, String>>),

    Connect(Option<HashMap<String, String>>),
    Disconnect(Option<HashMap<String, String>>),

    ClientUpdate(Option<HashMap<String, String>>),
    ClientInfo(Option<HashMap<String, String>>),
    ClientRemove(Option<HashMap<String, String>>),
    Client(Option<HashMap<String, String>>),

    Success(Option<HashMap<String, String>>),
    Error(Option<HashMap<String, String>>),
}

impl ToString for Commands {

    fn to_string(&self) -> std::string::String {
        let mut out_string = String::new();

        let (command, parameters) = match self {
            Commands::Request(arguments) => { ("!request:", arguments) },
            Commands::Info(arguments) => { ("!info:", arguments) },
            Commands::Connect(arguments) => { ("!connect:", arguments) },
            Commands::Disconnect(arguments) => { ("!disconnect:", arguments) },
            Commands::ClientUpdate(arguments) => { ("!clientUpdate:", arguments) },
            Commands::ClientInfo(arguments) => { ("!clientInfo:", arguments) },
            Commands::Client(arguments) => { ("!client:", arguments) },
            Commands::Error(arguments) => { ("!error:", arguments) },
            _ => { ("!error:", &None) }
        };

        out_string.push_str(command);

        if parameters.is_some() {
            let hash_map = parameters.borrow().as_ref().unwrap();
            for (k, v) in hash_map.iter() {
                out_string.push_str(" ");
                out_string.push_str(k.as_str());
                out_string.push_str(":");
                out_string.push_str(v.as_str())
            }
        }

        out_string
    }
}

impl From<&str> for Commands { 
    fn from(data: &str) -> Self {
        let regex = Regex::new(r###"(\?|!)([a-zA-z0-9]*):|([a-zA-z]*):([a-zA-Z0-9\-\+\[\]{}_=/]+|("(.*?)")+)"###).unwrap();
        let mut iter = regex.find_iter(data);
        let command = iter.next().unwrap().as_str();

        println!("command: {:?}", command);

        let mut map: HashMap<String, String> = HashMap::new();

        for i in iter {
            let parameter = i.as_str().to_string();
            let mut parts:Vec<&str> = parameter.split(":").collect();

            map.insert(parts.index(0).to_string(), parts.index(1).to_string());
        }

        let params = if map.capacity() > 1 {Some(map)} else { None };

        match command {
            "!request:" => Commands::Request(params),
            "!info:" => Commands::Info(params),

            "!connect:" => Commands::Connect(params),
            "!disconnect:" => Commands::Disconnect(params),

            "!clientUpdate:" => Commands::ClientUpdate(params),
            "!clientInfo:" => Commands::ClientInfo(params),
            "!client:" => Commands::Client(params),
            "!clientRemove:" => Commands::ClientRemove(params),
            
            "!success:" => Commands::Success(params),
            "!error:" => Commands::Error(params),
            
            _ => Commands::Error(params),
        }
    }
}

impl From<String> for Commands {
    fn from(data: String) -> Self {
        Commands::from(data.as_str())
    }
}

#[cfg(test)]
mod test_commands_v2 {
    use super::Commands;
    use std::collections::HashMap;

    #[test]
    fn test_creation_from_string() {
        let command_result = Commands::from("!connect: name:bop host:127.0.0.1 uuid:123456-1234-1234-123456");
        ()
    }

    #[test]
    fn test_to_string() {

        let mut a: HashMap<String, String> = HashMap::new();
        a.insert("name".to_string(), "michael".to_string());
        a.insert("host".to_string(), "127.0.0.1".to_string());
        a.insert("uuid".to_string(), "123456-1234-1234-123456".to_string());

        let command = Commands::Connect(Some(a));

        println!("{:?}", command.to_string())
    }
}
