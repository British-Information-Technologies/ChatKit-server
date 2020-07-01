use crate::server::client::client_profile::Client;
//use crate::client_management::client_profile::Client;

use std::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;
use dashmap::DashMap;

pub fn add_client(clients_ref: &Arc<Mutex<HashMap<String, Client>>>, client: &Client){
    let mut clients_hashmap = clients_ref.lock().unwrap();
    let uuid = client.get_uuid().to_string();
    clients_hashmap.insert(uuid, client.clone());
}
