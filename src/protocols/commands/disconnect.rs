use crate::client_management::client_profile::Client;

use std::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;

pub fn remove_client(clients_ref: &Arc<Mutex<HashMap<String,Client>>>, uuid: &String){
    let mut clients_hashmap = clients_ref.lock().unwrap();
    clients_hashmap.remove(uuid);
}
