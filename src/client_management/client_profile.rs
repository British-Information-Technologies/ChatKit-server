#[derive(Clone)]
pub struct Client{
    pub uuid: String,
    pub username: String,
    pub address: String,
}

impl Client{
    pub fn get_uuid(&self) -> &String{
        &self.uuid
    }

    pub fn get_username(&self) -> &String{
        &self.username
    }

    pub fn get_address(&self) -> &String{
        &self.address
    }
}
