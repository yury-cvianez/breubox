use dotenvy::dotenv;
use tokio::net::TcpStream;
use tiberius::{Client};
use tokio_util::compat::{Compat};

use crate::mndb::drivers::{mssql};


pub struct SchemaQueries {
    
    pub tables  : &'static str,
    pub keyp    : &'static str,
    pub keyf    : &'static str,
}

pub enum Dbs {
    
    SqlServer,
    //Postgresql,
}

impl Dbs {
    
    pub async fn get_cnn(&self) -> Result<Client<Compat<TcpStream>>, Box<dyn std::error::Error + Send + Sync>>  {
        
        dotenv().ok();
        
        match self {
            Dbs::SqlServer => {
                Ok(mssql::connect().await?)
            }
        }

    }

    pub fn get_schema(&self) {  }

}
