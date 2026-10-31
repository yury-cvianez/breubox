use dotenvy::dotenv;
use tiberius::ColumnData;
use tokio::net::TcpStream;
use tokio::runtime::{Builder, Runtime};
use tiberius::{Client, Config, AuthMethod};
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

use crate::mndb::dbs::{BoxError, Database, Row, Value};
use crate::aud::AuditDetails;


const TABLES        : &str = include_str!("sql/mssql/tables.sql");
const PRIMARY_KEYS  : &str = include_str!("sql/mssql/keyp.sql");
const FOREIGN_KEYS  : &str = include_str!("sql/mssql/keyf.sql");


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

    fn query(&mut self, sql: &str) -> Result<Vec<Row>, BoxError> {
        
        let Self { rt, client } = self;

        let rows = rt.block_on(async {
            client.query(sql, &[])
                .await?
                .into_first_result()
                .await
        })?;

        Ok(rows
            .into_iter()
            .map(|row| row.into_iter().map(to_value).collect())
            .collect()
        )
    }

    fn sql_tables(&self)       -> &'static str { TABLES }
    fn sql_primary_keys(&self) -> &'static str { PRIMARY_KEYS }
    fn sql_foreign_keys(&self) -> &'static str { FOREIGN_KEYS }

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

fn to_value(col: ColumnData<'_>) -> Value {

    match col {
        ColumnData::Bit(Some(v))             => Value::Bool(v),
        ColumnData::U8(Some(v))                => Value::Int(v as i64),
        ColumnData::I16(Some(v))              => Value::Int(v as i64),
        ColumnData::I32(Some(v))              => Value::Int(v as i64),
        ColumnData::I64(Some(v))              => Value::Int(v),
        ColumnData::F32(Some(v))              => Value::Float(v as f64),
        ColumnData::F64(Some(v))              => Value::Float(v),
        ColumnData::String(Some(v))  => Value::Text(v.into_owned()),

        _ => Value::Null,
    }
}
