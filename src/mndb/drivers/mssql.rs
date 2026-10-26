use tokio::net::TcpStream;
use tiberius::{Client, Config, AuthMethod};
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

use crate::aud::AuditDetails;



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

