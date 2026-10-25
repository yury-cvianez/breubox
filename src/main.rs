
pub mod aud;
pub mod mndb;

use crate::aud::AuditDetails;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // test record log
    // AuditDetails::new(
    //     "CD01".into(), 
        
    //     "ok".into(), 
    //     None
    // );

    // test mssql connection

    Ok(())   

}
