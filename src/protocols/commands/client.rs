use crate::client_management::client_profile::Client;

use std::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;

pub fn retrieve_requested_clients(clients_ref: &Arc<Mutex<HashMap<String,Client>>>, uuid: &String) -> String{
    //let address = String::new();
    //if data[1].starts_with("uuid:"){
    get_client_address(clients_ref, uuid)
    //}else if data[1].starts_with("name:"){
    //    address = ClientQ::get_clients_addresses();
    //}
}

fn get_client_address(clients_ref: &Arc<Mutex<HashMap<String,Client>>>, uuid: &String) -> String{
    // may not need to lock hashmap as it may cause difficulties later on
    let clients_hashmap = clients_ref.lock().unwrap();
    let client = clients_hashmap.get(uuid);
    match client{
        Some(data) => data.get_address().to_string(),
        None => String::from("client not online"),
    }
}
