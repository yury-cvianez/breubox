
pub mod aud;
pub mod mndb;

use crate::aud::AuditDetails;
use crate::mndb::mssql::cnn;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // test record log
    // AuditDetails::new(
    //     "CD01".into(), 
        
    //     "ok".into(), 
    //     None
    // );

    // test mssql connection
    mndb::init(); 
    cnn::connect().await?;

    Ok(())   

}
