struct Request {}

struct Info {}

struct Connect {}

struct Disconnect {}

struct ClientUpdate {}

struct ClientInfo {}

struct ClientRemove {}

struct Client {}

struct Success {}

struct Error {}

trait ClientRunnables {
    fn client_execution(client: &Client);
}

impl Runnables for Request {
    fn run() {
    }
}

impl ClientRunnables for Info {
    fn client_execution(client: &Client) {
        let params = client.get_server_info();
        let command = Commands::Success(Some(params));

        client.transmit_data(command.to_string().as_str());
    }
}

impl Runnables for Connect {
    fn run() {
    }
}

impl Runnables for Disconnect {
    fn run() {
    }
}

impl ClientRunnables for ClientUpdate {
    fn client_execution(client: &Client) {
        let mut command = Commands::Success(None);
        client.transmit_data(command.to_string().as_str());

        let data: HashMap<String, String> = [(String::from("uuid"), client.get_uuid())].iter().cloned().collect();
        let command = Commands::ClientUpdate(Some(data));

        self.server.update_all_clients(self.uuid.as_str(), command);

    }
}

impl Runnables for ClientInfo {
    fn run() {
    }
}

impl Runnables for ClientRemove {
    fn run() {
    }
}

impl Runnables for Client {
    fn run() {
    }
}

impl Runnables for Success {
    fn run() {
    }
}

impl Runnables for Error {
    fn run() {
    }
}
