use dotenvy::dotenv;
use tokio::net::TcpStream;
use tiberius::{Client, Config, AuthMethod};
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

use tokio::runtime::{Builder, Runtime};
use crate::mndb::dbs::{BoxError, Database, KeypInfo, KeyfInfo, TableInfo};
use crate::aud::AuditDetails;


const TABLES: &str = include_str!("sql/mssql/tables.sql");
const KEYP  : &str = include_str!("sql/mssql/keyp.sql");
const KEYF  : &str = include_str!("sql/mssql/keyf.sql");


pub struct Mssql {
    
    rt      : Runtime,
    client  : Client<Compat<TcpStream>>,
}

impl Mssql {

    pub fn connect() -> Result<Self, BoxError> {

        dotenv().ok();
        
        let rt = Builder::new_current_thread().enable_all().build()?;
        let client = rt.block_on(connect())?;

        Ok(Self {rt, client})
    }
}

impl Database for Mssql {

    fn tables(&mut self)        -> Result<Vec<TableInfo>, BoxError> {
        
        let Self {rt, client} = self;
        
        let rows = rt.block_on (async {
            client.query(TABLES, &[])
                .await?
                .into_first_result()
                .await
        })?;

        Ok(rows.iter().map(|r| TableInfo{
                schema  : r.get::<&str, _>(0).unwrap_or_default().to_string(),
                name    : r.get::<&str, _>(1).unwrap_or_default().to_string(),
        }).collect())

    }

}

pub async fn connect() -> Result<Client<Compat<TcpStream>>, Box<dyn std::error::Error + Send + Sync>>  {

    let mut config = Config::new();

    config.host(std::env::var("MSSQL_HT")?);
    config.port(std::env::var("MSSQL_PT")?.parse::<u16>()?);
    config.authentication(
        AuthMethod::sql_server(
            std::env::var("MSSQL_USR")?,
            std::env::var("MSSQL_PWD")?
        )
    );

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    config.trust_cert(); // remove after

    let client = Client::connect(
        config, 
        tcp.compat_write()
    ).await?;
        
    AuditDetails::new(
        "CD01".into(),
        "ok".into(),
        Some("Connection established".into())
    )?;
    
    Ok( client )
}
