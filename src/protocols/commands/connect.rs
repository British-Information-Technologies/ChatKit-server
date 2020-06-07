use crate::client_management::client_profile::Client;

use std::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;

pub fn add_client(clients_ref: &Arc<Mutex<HashMap<String,Client>>>, username: &String, uuid: &String, address: &String){
    let client = create_client_profile(username,uuid,address);
    let mut clients_hashmap = clients_ref.lock().unwrap();
    //let cloned_client = client.clone();
    clients_hashmap.insert(uuid.to_string(),client);
}

fn create_client_profile(username: &String, uuid: &String, address: &String) -> Client{
    Client{
        uuid: uuid.to_string(),
        username: username.to_string(),
        address: address.to_string(),
    }
}