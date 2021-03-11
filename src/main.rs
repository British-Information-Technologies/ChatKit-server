mod lib;

use clap::{App, Arg};
 
fn main() {
  let _args = App::new("--rust chat server--")
    .version("0.1.5")
    .author("Mitchel Hardie <mitch161>, Michael Bailey <michael-bailey>")
    .about("this is a chat server developed in rust, depending on the version one of two implementations will be used")
    .arg(
      Arg::new("config")
      .short('p')
      .long("port")
      .value_name("PORT")
      .about("sets the port the server listens on.")
      .takes_value(true))
    .get_matches();

  // creating the server object
}


// MARK: - general testing zone
// #[cfg(test)]
// mod tests {
//     use crate::server::server_profile::Server;
//     use crate::client_api::ClientApi;
//     use std::collections::HashMap;
//     use crate::commands::Commands;
//     use std::{thread, time};

//     #[test]
//     fn test_server_info() {
//         // setup the server
//         let name = "Server-01";
//         let address = "0.0.0.0:6000";
//         let owner = "noreply@email.com";

//         let mut server = Server::new(name, address, owner);
//         let result = server.start();

//         assert_eq!(result.is_ok(), true);

//         let dur = time::Duration::from_millis(1000);
//         thread::sleep(dur);
        
//         let api = ClientApi::get_info("127.0.0.1:6000");
//         assert_eq!(api.is_ok(), true);
//         if let Ok(api) = api {
//             println!("received: {:?}", api);
//             let mut map = HashMap::new();
//             map.insert("name".to_string(), name.to_string());
//             map.insert("owner".to_string(), owner.to_string());

//             let expected = Commands::Info(Some(map));
//             println!("expected: {:?}", expected);
//             assert_eq!(api, expected);
//         }
//     }

//     #[test]
//     fn test_server_connect() {
//         let name = "Server-01";
//         let address = "0.0.0.0:6001";
//         let owner = "noreply@email.com";

//         let mut server = Server::new(name, address, owner);
//         let _ = server.start().unwrap();

//         let api_result = ClientApi::new(address);
//         assert_eq!(api_result.is_ok(), true);
//         if api_result.is_ok() {
//             std::thread::sleep(std::time::Duration::from_secs(2));
//         }
//     }
// }

