use crate::server::client::client_profile::Client;
//use crate::client_management::client_profile::Client;

use std::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;

pub fn remove_client(clients_ref: &Arc<Mutex<HashMap<String, Client>>>, client: &Client){
    let mut clients_hashmap = clients_ref.lock().unwrap();
    clients_hashmap.remove(client.get_uuid()).unwrap();
}
