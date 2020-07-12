use crate::server::client::client_profile::Client;

pub fn format_client_data(uuid: &String, client: &Client) -> String{
    ["!client: username:",client.get_username(), " uuid:", uuid, " host:\"", client.get_address(), "\""].concat()
}
