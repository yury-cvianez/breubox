pub mod mssql;

use dotenvy::dotenv;


pub fn init() {  dotenv().ok(); }