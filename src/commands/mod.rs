use std::string::ToString;
use std::collections::HashMap;
use std::str::FromStr;

use std::borrow::Borrow;
use regex::Regex;
use std::ops::{Index};
use log::info;

// MARK: - command struct
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Commands {
    Request(Option<HashMap<String, String>>),
    Info(Option<HashMap<String, String>>),

    HeartBeat(Option<HashMap<String, String>>),

    Connect(Option<HashMap<String, String>>),
    Disconnect(Option<HashMap<String, String>>),

    ClientUpdate(Option<HashMap<String, String>>),
    ClientInfo(Option<HashMap<String, String>>),
    ClientRemove(Option<HashMap<String, String>>),
    Client(Option<HashMap<String, String>>),

    Success(Option<HashMap<String, String>>),
    Error(Option<HashMap<String, String>>),
}

#[derive(Debug)]
pub enum CommandParseError {
    UnknownCommand,
    NoString,
}


impl ToString for Commands {

    fn to_string(&self) -> std::string::String {
        let mut out_string = String::new();

        let (command, parameters) = match self {
            Commands::Request(arguments) => { ("!request:", arguments) },
            Commands::Info(arguments) => { ("!info:", arguments) },
            Commands::HeartBeat(arguments) => {("!heartbeat:", arguments)},
            Commands::Connect(arguments) => { ("!connect:", arguments) },
            Commands::Disconnect(arguments) => { ("!disconnect:", arguments) },
            Commands::ClientUpdate(arguments) => { ("!clientUpdate:", arguments) },
            Commands::ClientInfo(arguments) => { ("!clientInfo:", arguments) },
            Commands::ClientRemove(arguments) => { ("!clientRemove", arguments) }
            Commands::Client(arguments) => { ("!client:", arguments) },
            Commands::Success(arguments) => { ("!success:", arguments) },
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

                if v.contains(":") {
                    out_string.push_str(format!("\"{}\"",v.as_str()).as_str())
                } else {
                    out_string.push_str(v.as_str());
                }
            }
        }
        out_string
    }
}

impl FromStr for Commands {
    type Err = CommandParseError;

    fn from_str(data: &str) -> std::result::Result<Self, Self::Err> {
        let regex = Regex::new(r###"(\?|!)([a-zA-z0-9]*):|([a-zA-z]*):([a-zA-Z0-9@\-\+\[\]{}_=/.]+|("(.*?)")+)"###).unwrap();
        let mut iter = regex.find_iter(data);
        let command_opt = iter.next();

        if command_opt.is_none() {
            return Err(CommandParseError::NoString);
        }
        let command = command_opt.unwrap().as_str();


        println!("command parsed to: {:?}", command);

        let mut map: HashMap<String, String> = HashMap::new();

        for i in iter {
            let parameter = i.as_str().to_string();
            let parts:Vec<&str> = parameter.split(":").collect();

            map.insert(parts.index(0).to_string(), parts.index(1).to_string());
        }

        let params = if map.capacity() > 0 {Some(map)} else { None };

        Ok(match command {
            "!request:" => Commands::Request(params),
            "!info:" => Commands::Info(params),

            "!heartbeat:" => Commands::HeartBeat(params),

            "!connect:" => Commands::Connect(params),
            "!disconnect:" => Commands::Disconnect(params),

            "!clientUpdate:" => Commands::ClientUpdate(params),
            "!clientInfo:" => Commands::ClientInfo(params),
            "!client:" => Commands::Client(params),
            "!clientRemove:" => Commands::ClientRemove(params),
            
            "!success:" => Commands::Success(params),
            "!error:" => Commands::Error(params),
            
            _ => Commands::Error(None),
        })
    }
}

impl From<String> for Commands {
    fn from(data: String) -> Self {
        if let Ok(data) = data.as_str().parse() {
            data
        } else {
            info!("Command: failed to parse with");
            Commands::Error(None)
        }
    }
}

impl From<&[u8; 1024]> for Commands {
    fn from(data: &[u8; 1024]) -> Self {
        let incoming_message = String::from(String::from_utf8_lossy(data));
        Commands::from(incoming_message)
    }
}

#[cfg(test)]
mod test_commands_v2 {
    #![feature(test)]
    extern crate test;
    use super::Commands;
    use std::collections::HashMap;
    use test::Bencher;
    use std::str::FromStr;
    use super::CommandParseError;

    #[test]
    fn test_creation_from_string() {
        let command_result = Commands::from_str("!connect: name:bop host:127.0.0.1 uuid:123456-1234-1234-123456");
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

    #[bench]
    fn benchmark(b: &mut Bencher) {
        b.iter(|| {let a = Commands::from_str("!connect: host:192.168.0.1 name:\"michael-bailey\" uuid:123456-1234-1234-123456").unwrap();})
    }
}
