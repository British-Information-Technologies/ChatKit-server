pub struct Client{
    uuid: String,
    username: String,
    address: String,
}

impl Client{
    pub fn get_uuid(&self) -> String{
        self.uuid
    }

    pub fn get_username(&self) -> String{
        self.username
    }

    pub fn get_addres(&self) -> String{
        self.address
    }
}
