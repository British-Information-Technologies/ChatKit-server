#![feature(in_band_lifetimes)]

mod client_api;
mod commands;
mod server;
mod lib;

use cursive::{
    Cursive,
    menu::*,
    event::Key,
    CursiveExt,
    align::Align,
    view::SizeConstraint,
    event::Event,
    XY,
};

use std::{
    time::Duration,
    sync::{
        Arc,
        Mutex
    }
};
use crossterm::ErrorKind;
use log::info;
use clap::{App, Arg};


use crate::server::ServerV3::Server;
use cursive::views::{Dialog, TextView, Menubar, LinearLayout, ResizedView, ListView, Panel};
use crate::server::ui::server_view_controller::ServerControlView;

fn main() -> Result<(), ErrorKind> {
    let args = App::new("--rust chat server--")
        .version("0.1.5")
        .author("Mitchel Hardie <mitch161>, Michael Bailey <michael-bailey>")
        .about("this is a chat server developed in rust, depending on the version one of two implementations will be used")
        .arg(Arg::with_name("graphical")
            .short('g')
            .takes_value(false)
            .about("Enables graphical mode"))
        .get_matches();

    if args.is_present("graphical") {

        let server = Server::new("server-001", "0.0.0.0:6000", "michael bailey");

        ServerControlView::new(server.unwrap());
        Ok(())
    } else {
        let mut server = crate::server::server_profile::Server::new("Server-01", "0.0.0.0:6000", "noreply@email.com");

        server.start()?;
        loop { std::thread::sleep(Duration::from_secs(1)); }
    }
}



// MARK: - general testing zone
#[cfg(test)]
mod tests {
    use crate::server::server_profile::Server;
    use crate::client_api::ClientApi;
    use std::collections::HashMap;
    use crate::commands::Commands;
    use std::{thread, time};

    #[test]
    fn test_server_info() {
        // setup the server
        let name = "Server-01";
        let address = "0.0.0.0:6000";
        let owner = "noreply@email.com";

        let mut server = Server::new(name, address, owner);
        let result = server.start();

        assert_eq!(result.is_ok(), true);

        let dur = time::Duration::from_millis(1000);
        thread::sleep(dur);
        
        let api = ClientApi::get_info("127.0.0.1:6000");
        assert_eq!(api.is_ok(), true);
        if let Ok(api) = api {
            println!("received: {:?}", api);
            let mut map = HashMap::new();
            map.insert("name".to_string(), name.to_string());
            map.insert("owner".to_string(), owner.to_string());

            let expected = Commands::Info(Some(map));
            println!("expected: {:?}", expected);
            assert_eq!(api, expected);
        }
    }

    #[test]
    fn test_server_connect() {
        let name = "Server-01";
        let address = "0.0.0.0:6001";
        let owner = "noreply@email.com";

        let mut server = Server::new(name, address, owner);
        let _ = server.start().unwrap();

        let api_result = ClientApi::new(address);
        assert_eq!(api_result.is_ok(), true);
        if api_result.is_ok() {
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
    }
}