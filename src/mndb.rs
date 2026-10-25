pub mod mssql;

use dotenvy::dotenv;
use tiberius::{Client};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat};

use crate::mndb::mssql::cnn;



pub enum Dbs {
    
    SqlServer,
    //Postgresql,
}

pub enum Conn {
    SqlServer(Client<Compat<TcpStream>>),
    // Postgres(tokio_postgres::Client),
}

pub struct SchemaQueries {
    
    pub tables  : &'static str,
    pub keyp    : &'static str,
    pub keyf    : &'static str,
}



pub fn load_env() { dotenv().ok(); }

impl Dbs {
    
    pub async fn get_cnn(&self) -> Result<Conn, Box<dyn std::error::Error + Send + Sync>>  {
        
        load_env();
        
        match self {
            Dbs::SqlServer => Ok(Conn::SqlServer(cnn::connect().await?)),

        }

    }

    pub fn get_schema(&self) {  }

}
