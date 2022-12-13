use crate::Address;

struct User {
    name:String,
    email:String,
    password:String,
    address:Address::Address
}

impl User {
    fn get_instance(name:String, email:String,password:String,address:Address::Address) -> User {
        User{name:name,email:email,password:password,address:address}
    }
}