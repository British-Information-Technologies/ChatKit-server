pub fn get_server_info() -> String{
    let mut server_details = "".to_string();
    let server_name = String::from("Server-01");
    let server_owner = String::from("mickyb18");

    server_details.push_str(&server_name.to_string());
    server_details.push_str(&" ".to_string());
    server_details.push_str(&server_owner.to_string());

    server_details
}
