pub struct Address {
    address:String,
    postal_code:i32,
    phone_number:String
}

impl Address {
    pub fn address(&self) -> &str {
        self.address.as_ref()
    }

    pub fn postal_code(&self) -> &str {
        self.postal_code.as_ref()
    }

    pub fn phone_number(&self) -> &str {
        self.phone_number.as_ref()
    }

}