pub struct ServerControlView {
    display: Cursive,

    // MARK: - ViewModel stuff

    server_name: String,
    server_host: String,
    server_owner: String,

    client_list: Vec<String>,
    running: String,
}

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
        v.display.add_global_callback(Event::CtrlChar('a'), |s| s.add_layer(About::new()));

        // TODO: - this will be tied to the server run function
        // v.display.add_global_callback(Event::Refresh, |s| s.user_data::<Arc<Server>>().unwrap().);

    fn get_display_channel() -> CbSink {
        Cursive::default().cb_sink().clone()
    }
}
