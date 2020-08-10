use std::string::ToString;
use std::collections::HashMap;
use dashmap::DashMap;
use std::borrow::Borrow;
use regex::Regex;
use std::ops::Index;
use zeroize::Zeroize;

#[derive(Clone, Debug)]
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
    fn compare_params(&self, params: &Option<HashMap<String, String>>, other_params: &Option<HashMap<String, String>>) -> bool {
        match (params, other_params) {
            (None, Some(_other_params)) => false,
            (Some(_params), None) => false,
            (None, None) => true,
            (Some(params), Some(other_params)) => {
                let mut result = false;
                
                if params.len() == other_params.len() {
                    for (key, value) in params.iter() {
                        if let Some(other_value) = other_params.get(key) {
                            if value != other_value {
                                result = false;
                                break;
                            } else {
                                result = true;
                            }
                        }
                    }
                }

                result
            },
        }
    }
}

impl PartialEq for Commands {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Commands::Request(params), Commands::Request(other_params)) => self.compare_params(&params, &other_params),
            (Commands::Info(params), Commands::Info(other_params)) => self.compare_params(&params, &other_params),
            (Commands::Connect(params), Commands::Connect(other_params)) => self.compare_params(&params, &other_params),
            (Commands::Disconnect(params), Commands::Disconnect(other_params)) => self.compare_params(&params, &other_params),
            (Commands::ClientUpdate(params), Commands::ClientUpdate(other_params)) => self.compare_params(&params, &other_params),
            (Commands::ClientInfo(params), Commands::ClientInfo(other_params)) => self.compare_params(&params, &other_params),
            (Commands::ClientRemove(params), Commands::ClientRemove(other_params)) => self.compare_params(&params, &other_params),
            (Commands::Client(params), Commands::Client(other_params)) => self.compare_params(&params, &other_params),
            (Commands::Success(params), Commands::Success(other_params)) => self.compare_params(&params, &other_params),
            (Commands::Error(params), Commands::Error(other_params)) => self.compare_params(&params, &other_params),
            _ => false,
        }
    }

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
            let parts:Vec<&str> = parameter.split(":").collect();

            map.insert(parts.index(0).to_string(), parts.index(1).to_string());
        }

        let params = if map.capacity() > 0 {Some(map)} else { None };

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

/*impl From<&[u8; 1024]> for Commands {
    fn from(data: &[u8; 1024]) -> Self {
        let incoming_message = String::from(String::from_utf8_lossy(data));
        data.zeroize();
        Commands::from(incoming_message.as_str())
    }
}*/

impl From<&mut [u8; 1024]> for Commands {
    fn from(data: &mut [u8; 1024]) -> Self {
        let incoming_message = String::from(String::from_utf8_lossy(data));
        data.zeroize();
        Commands::from(incoming_message.as_str())
    }
}

/*#[cfg(test)]
mod test_commands_v2 {
    #![feature(test)]
    //extern crate test;
    use super::Commands;
    use std::collections::HashMap;
    use test::Bencher;

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

    #[bench]
    fn benchmark(b: &mut Bencher) {
        b.iter(|| Commands::from("!connect: host:192.168.0.1 name:\"michael-bailey\" uuid:123456-1234-1234-123456"))
    }
}*/
