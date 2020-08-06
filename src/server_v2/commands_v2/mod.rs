use std::collections::HashMap;
use std::borrow::Borrow;
use regex::Regex;
use std::ops::Index;

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

impl Commands {
    pub fn to_String(&self) -> String {

        let mut out_string = String::new();

        let (command, parameters) = match self {
            Commands::Info(arguments) => { ("!info:", arguments) },
            Commands::Connect(arguments) => { ("!connect:", arguments) },
            Commands::Disconnect(arguments) => { ("!disconnect:", arguments) },
            Commands::ClientUpdate(arguments) => { ("!clientUpdate:", arguments) },
            Commands::ClientInfo(arguments) => { ("!clientInfo:", arguments) },
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

    pub fn from_string(data: &str) -> Result<Commandsv2, &'static str> {
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
            "!info:" => Ok(Commands::Info(params)),
            "!connect:" => Ok(Commands::Connect(params)),
            "!clientInfo:" => Ok(Commands::ClientInfo(params)),
            "!clientUpdate:" => Ok(Commands::ClientUpdate(params)),
            "!disconnect:" => Ok(Commands::Disconnect(params)),
            "!error:" => Ok(Commands::Error(params)),
            _ => { Err("NOT IMPLEMENTED") }
        }
    }
}

#[cfg(test)]
mod test_commands_v2 {
    use crate::server_v2::commands_v2::Commandsv2;
    use std::collections::HashMap;

    #[test]
    fn test_creation_from_string() {
        let command_result = Commandsv2::from_string("!connect: name:bop host:127.0.0.1 uuid:123456-1234-1234-123456");
        assert!(command_result.is_ok(), true);
        let command = command_result.unwrap_or(Commandsv2::Error(None));
        ()
    }

    #[test]
    fn test_to_string() {

        let mut a: HashMap<String, String> = HashMap::new();
        a.insert("name".to_string(), "michael".to_string());
        a.insert("host".to_string(), "127.0.0.1".to_string());
        a.insert("uuid".to_string(), "123456-1234-1234-123456".to_string());

        let command = Commandsv2::Connect(Some(a));

        println!("{:?}", command.to_String())
    }
}