use cursive::{CbSink, Cursive, CursiveExt};

use crate::server::server_v3::Server;
use crate::server::ui::about_panel::about;
use crate::server::ui::main_menu::main_menu;
use cursive::event::Event;

#[allow(dead_code)]
pub enum UpdateTypes {
    AddClient()
}

/// # ServerViewConroller
///
/// This Struct contains all the controller logic to allow the server to interact with the view
#[allow(dead_code)]
pub struct ServerViewController {
    display: Cursive,

    server_name: String,
    server_host: String,
    server_owner: String,

    client_list: Vec<String>,
    running: String,
}

#[allow(dead_code)]
impl ServerViewController {
    pub fn new(server: Server) {

        let mut v = Self {
            display: Cursive::default(),
            server_name: server.get_name().to_string(),
            server_host: server.get_address().to_string(),
            server_owner: server.get_owner().to_string(),
            client_list: Vec::new(),
            running: "None".to_string()
        };

        // set global shortcuts
        v.display.add_global_callback(Event::CtrlChar('q'), |s| s.quit());
        v.display.add_global_callback(Event::CtrlChar('a'), |s| s.add_layer(about()));

        // seting up menubar
        v.display.menubar().add_subtree("Server", main_menu());
        v.display.set_autohide_menu(false)

        // setup the display menubar.

        // TODO: - this will be tied to the server run function
        // v.display.add_global_callback(Event::Refresh, |s| s.user_data::<Arc<Server>>().unwrap().);

    }


    fn get_display_channel() -> CbSink {
        Cursive::default().cb_sink().clone()
    }
}
