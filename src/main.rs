mod client_api;
mod commands;
mod server;
mod lib;

use cursive::{
    Cursive,
    menu::*,
    event::Key,
    views::{ Dialog, TextView, LinearLayout, ListView, ResizedView, Panel },
    CursiveExt,
    align::Align,
    view::SizeConstraint,
};
//use std::sync::Arc;
use std::time::Duration;
use std::sync::Arc;
use crossterm::ErrorKind;
use log::info;
use clap::{App, Arg};

use crate::server::server_profile::Server;

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
        let server = Server::new("Server-01", "0.0.0.0:6000", "noreply@email.com");
        let server_arc = Arc::new(server);
        let s1 = server_arc.clone();
        let s2 = s1.clone();

        cursive::logger::init();

        info!("Main: init display");
        let mut display = Cursive::default();

        info!("Main: setting up callbacks");
        display.add_global_callback(Key::Backspace, |s| s.quit());
        display.add_global_callback(Key::Tab, |s| s.toggle_debug_console());
        display.add_global_callback(Key::Esc, |s| s.select_menubar());

        info!("Main: setting up menu bar");
        let _ = display.menubar()
            .add_subtree("Server",
                         MenuTree::new()
                             .leaf("about",
                                   |s| s.add_layer(about()))
                             .delimiter()
                             .leaf("quit", |s| s.quit()))
            .add_subtree("File",
                         MenuTree::new()
                             .leaf("Start", move |_s| {let _ = s1.start();})
                             .leaf("Stop", move |_s| {let _ = s2.stop();})
                             .delimiter()
                             .leaf("Debug", |s| {s.toggle_debug_console();}));
        info!("Main: entering loop");
        display.add_layer(control_panel());
        display.run();
        Ok(())
    } else {
        let server = Server::new("Server-01", "0.0.0.0:6000", "noreply@email.com");

        server.start()?;
        loop {std::thread::sleep(Duration::from_secs(1));}
    }
}

fn about() -> Dialog {
    Dialog::new()
        .content(TextView::new("Rust-Chat-Server\nmade by\n Mitchell Hardie\nMichael Bailey\nMit Licence")
        ).button("Close", |s| {let _ = s.pop_layer(); s.add_layer(control_panel())} )
}

#[allow(dead_code)]
fn launch_screen() -> Dialog {
    Dialog::new()
        .content(TextView::new("\
        Server.
        * press <ESC> for menu bar
        * press <TAB> for debug (FIXME)
        * press <DEL> to exit.
        ").align(Align::center()))
        .button("ok", |s| {s.pop_layer();})
}

fn control_panel() -> ResizedView<Panel<LinearLayout>> {

    let mut root = LinearLayout::horizontal();
    let mut left = LinearLayout::vertical();
    let mut right = ListView::new();
    right.add_child("test", TextView::new(""));
    right.add_child("test", TextView::new(""));
    right.add_delimiter();
    right.add_child("test", TextView::new(""));
    right.add_child("test", TextView::new(""));

    left.add_child(TextView::new("Hello world"));

    root.add_child(ResizedView::new(SizeConstraint::Full, SizeConstraint::Full, Panel::new(left)));
    root.add_child(ResizedView::new(SizeConstraint::Full, SizeConstraint::Full, Panel::new(right)));
    ResizedView::new(SizeConstraint::Fixed(60), SizeConstraint::Fixed(18), Panel::new(root))
}

// MARK: - general testing zone
#[cfg(test)]
mod tests {
    use crate::server::server_profile::Server;
    use crate::client_api::ClientApi;
    use std::collections::HashMap;
    use crate::commands::Commands;
    use std::{thread, time};
    use std::time::Duration;

    #[test]
    fn test_server_info() {
        // setup the server
        let name = "Server-01";
        let address = "0.0.0.0:6000";
        let owner = "noreply@email.com";

        let server = Server::new(name, address, owner);
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

        let server = Server::new(name, address, owner);
        let _ = server.start().unwrap();

        let api_result = ClientApi::new(address);
        assert_eq!(api_result.is_ok(), true);
        if let Ok(api) = api_result {
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
    }
}

#[cfg(test)]
mod crypto_tests {
    use openssl::rsa::{Rsa, Padding};
    use openssl::ssl::{SslMethod, SslAcceptor, SslStream, SslFiletype, SslConnector, SslVerifyMode};
    use std::net::{TcpListener, TcpStream};
    use std::sync::Arc;
    use std::thread;
    use std::str;

    #[test]
    // MARK: - working encryption example for rsa
    fn gen_rsa() {
        let rsa = Rsa::generate(1024).unwrap();

        let ref1 = rsa.public_key_to_pem().unwrap();
        let ref2 = rsa.private_key_to_pem().unwrap();

        let public = str::from_utf8(&ref1).unwrap().to_string();
        let private = str::from_utf8(&ref2).unwrap().to_string();

        println!("public key size: {}", public.len());
        println!("{}", public);

        println!("private key size: {}", private.len());
        println!("{}", private);

        let data = b"this is a sentence";
        println!("before: {:?}", data);

        let mut buf = vec![0; rsa.size() as usize];
        let encrypted_len = rsa.private_encrypt(data, &mut buf, Padding::PKCS1).unwrap();
        println!("during: {:?}", &buf);

        let mut buf2 = vec![0; rsa.size() as usize];
        let _ = rsa.public_decrypt(&mut buf, &mut buf2, Padding::PKCS1).unwrap();
        println!("after: {:?}", &buf2);
    }

    #[test]
    fn tls_handshake() {
        // spawn the server
        thread::spawn(|| {
            println!("creating acceptor");
            let mut acceptor = SslAcceptor::mozilla_modern(SslMethod::tls()).unwrap();
            acceptor.set_private_key_file("cert.pem", SslFiletype::PEM).unwrap();
            acceptor.set_certificate_chain_file("root.pem").unwrap();
            acceptor.check_private_key().unwrap();
            let acceptor = Arc::new(acceptor.build());

            let listener = TcpListener::bind("0.0.0.0:6000").unwrap();

            println!("entering loop");
            loop {
                for stream in listener.incoming() {
                    println!("client accepted");
                    match stream {
                        Ok(stream) => {
                            let acceptor = acceptor.clone();
                            thread::spawn(move || {
                                let mut stream = acceptor.accept(stream).unwrap();
                                
                                let mut buffer: [u8; 1024] = [0; 1024];

                                stream.ssl_read(&mut buffer).unwrap();
                                let result = str::from_utf8(&buffer).unwrap();
                                if buffer == "echo".as_bytes() {
                                    let _ = stream.ssl_write("echo".as_bytes()).unwrap();
                                }
                            });
                        }
                        Err(e) => { /* connection failed */ }
                    }
                }
            }
        });

        let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();

        let stream = TcpStream::connect("localhost:6000").unwrap();
        let mut stream = connector.connect("127.0.0.1", stream).unwrap();

        let _ = stream.ssl_write("echo".as_bytes()).unwrap();
    }
}