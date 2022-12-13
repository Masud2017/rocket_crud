use rocket::*;

#[get("/test")]
pub fn test_method_from_test_controller() -> String {"Hello world this is a test code from test controller".to_string()}