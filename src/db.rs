pub mod models;


use diesel::prelude::*;

pub fn main(){
    establish_connection();
    println!("Hello, world!");
}

pub fn establish_connection() -> SqliteConnection {
    let database_url = "temp/test.db";
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}n