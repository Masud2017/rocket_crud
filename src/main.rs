#[macro_use] extern crate rocket;
use serde::{Serialize,Deserialize};
use rocket::log::{info_, private::info};

// user defined crate area started
extern crate service;
extern crate controller;

use service::AuthService;
use controller::TestController;
// user defined crate area ended


#[get("/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
        format!("Hello, {} year old named {}!", age, name)
}

#[derive(Serialize, Deserialize, Debug)]
struct Entity {
    name:String,
    age:i32,
    active:bool
}

impl ToString for Entity {
    fn to_string(&self) -> String {
        let st = String::from("{self}");
        return st;
    }
}

#[get("/")]
fn get_entity() -> String {
    AuthService::test_method();
    info!("Yo this is a logger");
    let en:Entity = Entity{
        name:"masud karim".to_string(),
        age:23,
        active:true
    };
    let serialized_json_data = serde_json::to_string(&en).unwrap();
    return serialized_json_data;
}

#[launch]
fn rocket() -> _ {
        rocket::build().mount("/hello", routes![hello,get_entity,TestController::test_method_from_test_controller])
}
