use crate::client_management::client_profile::Client;

// send a list of all clients.

// client !clientupdate:
// server !client: name:"vobo" uuid:24653526-23464562-46346-3563563 host:"127.0.0.1"
// server !client: name:"bovo" uuid:24643526-23464562-46346-3563563 host:"127.0.0.1"
pub fn format_client_data(uuid: &String, client: &Client) -> String{
    ["!client: ",client.get_username(), uuid, " host:\"", client.get_address(), "\""].concat()
}
