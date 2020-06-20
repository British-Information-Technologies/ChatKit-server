use crate::client_management::client_profile::Client;

use std::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;

pub fn get_client_data(clients_ref: &Arc<Mutex<HashMap<String,Client>>>, uuid: &String) -> String{
    let clients_hashmap = clients_ref.lock().unwrap();
    let client = clients_hashmap.get(uuid);
    match client{
        Some(data) => {
            let mut message = String::from("!success: ");
            message.push_str(&data.get_uuid().to_string());
            message.push_str(&" host:".to_string());
            message.push_str(&data.get_address().to_string());
            message.push_str(&" ".to_string());
            message.push_str(&data.get_username().to_string());
            message
        },
        None => String::from("client not online"),
    }
}
