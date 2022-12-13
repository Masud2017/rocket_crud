/**
 * This module is maintain all the database related opeartions and also contain a factory method that will help crafting different
 * db object based on the need.
 * 
 * @author Masud karikm
 */

use std::fmt::Display;
use std::fmt::Formatter;
use std::result::Result;
use std::fmt::Error;

pub enum db_type {
    MYSQL,
    POSTGRES,
    MONGO
}

// implementing the display trait for the `db_type` enum
impl Display for db_type {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            db_type::MYSQL => {
                println!("MYSQL");
                Result::Ok(())
            },
            db_type::POSTGRES => {
                println!("POSTGRES");
                Result::Ok(())
            },
            db_type::MONGO => {
                println!("MONGO");
                Result::Ok(())
            }
        }
    }
}

pub struct db_obj {
    db_type:db_type,

}