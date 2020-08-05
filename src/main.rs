#![feature(test)]

mod client_api;
mod commands;
mod server;


use crate::server::server_profile::Server;
use client_api::ClientApi;
use crossterm::ErrorKind;
use cursive::{
    Cursive,
    menu::*,
    event::Key,
    views::{ Dialog, TextView, LinearLayout, ListView, ResizedView, Panel },
    Rect,
    CursiveExt,
    align::{Align, HAlign},
    view::SizeConstraint,
};
use std::sync::Arc;
use log::info;

fn main() -> Result<(), ErrorKind> {
    let server = Server::new("Server-01", "0.0.0.0:6000", "noreply@email.com");
    let server_arc = Arc::new(server);
    let s1 = server_arc.clone();
    let s2 = s1.clone();

    cursive::logger::init();

    info!("Main: init Display");
    let mut Display = Cursive::default();

    info!("Main: setting up callbacks");
    Display.add_global_callback(Key::Backspace, |s| s.quit());
    Display.add_global_callback(Key::Tab, |s| s.toggle_debug_console());
    Display.add_global_callback(Key::Esc, |s| s.select_menubar());

    info!("Main: setting up menu bar");
    let _ = Display.menubar()
        .add_subtree("Server",
                     MenuTree::new()
                         .leaf("About",
                               |s| s.add_layer(About()))
                         .delimiter()
                         .leaf("quit", |s| s.quit()))
        .add_subtree("File",
                     MenuTree::new()
                         .leaf("Start", move |s| {s1.start();})
                         .leaf("Stop", move |s| {s2.stop();})
                         .delimiter()
                         .leaf("Debug", |s| {s.toggle_debug_console();}));
    info!("Main: entering loop");
    Display.add_layer(Control_Panel());
    Display.run();
    Ok(())
}

fn About() -> Dialog {
    Dialog::new()
        .content(TextView::new("Rust-Chat-Server\nmade by\n Mitchell Hardie\nMichael Bailey\nMit Licence")
        ).button("Close", |s| {let _ = s.pop_layer(); s.add_layer(Control_Panel())} )
}

fn Launch_screen() -> Dialog {
    Dialog::new()
        .content(TextView::new("\
        Server.
        * press <ESC> for menu bar
        * press <TAB> for debug (FIXME)
        * press <DEL> to exit.
        ").align(Align::center()))
        .button("ok", |s| {s.pop_layer();})
}

fn Control_Panel() -> ResizedView<Panel<LinearLayout>> {

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
    ResizedView::new(SizeConstraint::Fixed(64), SizeConstraint::Fixed(20), Panel::new(root))
}

// MARK: - general testing zone
#[cfg(test)]
mod tests {
    #![feature(test)]
    use super::Server;
    use crate::client_api::ClientApi;
    use std::thread::spawn;
    use std::collections::HashMap;
    use crate::commands::Commands;

    #[test]
    fn test_server_info() {
        // setup the server
        let name = "Server-01";
        let address = "0.0.0.0:6000";
        let owner = "noreply@email.com";

        let server = Server::new(name, address, owner);
        let _ = server.start().unwrap();

        let api = ClientApi::get_info("127.0.0.1:6000");
        if api.is_some() {

            let mut map = HashMap::new();
            map.insert("name".to_string(), name.to_string());
            map.insert("owner".to_string(), owner.to_string());

            let expected = Commands::Info(Some(map));


            let api = api.unwrap();
            assert_eq!(api, expected);
        } else {
            return
        }
    }
}