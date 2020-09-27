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


fn gen_ui() {
    // MARK: - setup the server.
    info!("Main: init Server");
    let server = Server::new("Server-01", "0.0.0.0:6000", "noreply@email.com");
    let server_arc = Arc::new(Mutex::new(server));

    info!("Main: init display");
    let mut display = Cursive::default();

    info!("Main: init cursive logger");
    cursive::logger::init();

    info!("Main: setting user data");
    display.set_user_data(server_arc);

    // MARK: - setup callbacks
    info!("Main: setting up callbacks");
    display.add_global_callback(Key::Backspace, |s| s.quit());
    display.add_global_callback(Key::Tab, |s| s.toggle_debug_console());
    display.add_global_callback(Key::Esc, |s| s.select_menubar());
    display.set_autohide_menu(false);
    display.add_global_callback(Event::WindowResize, |s| {
        info!("Display: resized!");
        std::process::Command::new("open").args(&["-a","Terminal"]).output().expect("not on mac os");
        let _ = s.pop_layer();
        let p = control_panel(s.screen_size(), s.user_data::<Arc<Mutex<Server>>>().unwrap().clone());
        s.add_layer(p);
        s.refresh();
    });
    display.set_autorefresh(true);


    info!("Main: getting sender and pushing events");
    let mut sender = display.cb_sink();
    sender.send(Box::new(|s| {
        menu_bar(s.menubar());
        s.add_layer(launch_screen());
    }));

    info!("Main: entering loop");
    display.run();
}

fn about() -> Dialog {
    Dialog::new()
        .content(TextView::new("Rust-Chat-Server\nmade by\n Mitchell Hardie\nMichael Bailey\nMit Licence")
                .align(Align::center()))
        .button("Close", |s| {
            let _ = s.pop_layer();
        })
}

#[allow(dead_code)]
fn launch_screen() -> Dialog {
    Dialog::new()
        .content(TextView::new("\
        Welcome.

        --- Controls ---
        * press <ESC> for menu bar
        * press <TAB> for debug (FIXME)
        * press <DEL> to exit.
        ").align(Align::top_left()))
        .button("ok", |s| {
            s.pop_layer();
            let p = control_panel(s.screen_size(), s.user_data::<Arc<Mutex<Server>>>().unwrap().clone());
            s.add_layer(p);
        })
}

fn menu_bar(bar: &mut Menubar) {
    bar.add_subtree("Server",
                         MenuTree::new()
                             .leaf("about",
                                   |s| s.add_layer(about()))
                             .delimiter()
                             .leaf("quit", |s| s.quit()))
            .add_subtree("File",
                         MenuTree::new()
                             .leaf("Start", |s| {

                                    let user_data_option = s.user_data::<Arc<Mutex<Server>>>();

                                    if let Some(user_data) = user_data_option {
                                        let arc = user_data.clone();
                                        let lock_result = arc.lock();
                                        if let Ok(mut server) = lock_result {
                                            let _ = server.start();
                                            let _ = s.pop_layer();
                                            let p = control_panel(s.screen_size(), s.user_data::<Arc<Mutex<Server>>>().unwrap().clone());
                                            s.add_layer(p);
                                        }
                                    }
                                })
                             .leaf("Stop", |s| {
                                    let user_data_option = s.user_data::<Arc<Mutex<Server>>>();

                                    if let Some(user_data) = user_data_option {
                                        let arc = user_data.clone();
                                        let lock_result = arc.lock();
                                        if let Ok(mut server) = lock_result {
                                            let _ = server.stop();
                                            let _ = s.pop_layer();
                                            let p = control_panel(s.screen_size(), s.user_data::<Arc<Mutex<Server>>>().unwrap().clone());
                                            s.add_layer(p);
                                        }
                                    }
                                })
                             .delimiter()
                             // TODO: - create custom debug console
                             .leaf("Debug", |s| {s.toggle_debug_console();}));
}

fn control_panel(screen_size: XY<usize>, server_arc: Arc<Mutex<Server>>) -> ResizedView<Panel<LinearLayout>> {
    let mut root = LinearLayout::horizontal();
    let mut left = LinearLayout::vertical();
    let mut right = ListView::new();
    
    right.add_child("test", TextView::new(""));
    right.add_delimiter();
    right.add_child("test", TextView::new(""));
    right.add_child("test", TextView::new(""));

    left.add_child(TextView::new("---| Server |---"));
    left.add_child(TextView::new(format!("name: {}", server_arc.lock().unwrap().name)));
    left.add_child(TextView::new(format!("owner: {}", server_arc.lock().unwrap().author)));
    left.add_child(TextView::new(format!("host: {}", server_arc.lock().unwrap().address)));
    left.add_child(TextView::new(format!("running: {}", server_arc.lock().unwrap().running)));
    left.add_child(TextView::new(format!("screen size: {:?}", screen_size)));

    root.add_child(ResizedView::new(SizeConstraint::AtLeast(30), SizeConstraint::Full, Panel::new(left)));
    root.add_child(ResizedView::new(SizeConstraint::Full, SizeConstraint::Full, Panel::new(right)));
    ResizedView::new(SizeConstraint::Fixed(screen_size.x-4), SizeConstraint::Fixed(screen_size.y-4), Panel::new(root))
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