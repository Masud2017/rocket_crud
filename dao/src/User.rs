struct User {
    name:String,
    email:String,
    password:String,
    address:Address
}

impl User {
    fn get_instance(name:String, email:String,password:String) -> User {
        User{name:name,email:email,password:password}
    }
}