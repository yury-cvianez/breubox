
pub mod aud;
pub mod mndb;

use crate::aud::AuditDetails;
use crate::mndb::dbs::{BoxError,Dbs};


fn main() -> Result<(), BoxError> {

    // test record log
    // AuditDetails::new(
    //     "CD01".into(), 
        
    //     "ok".into(), 
    //     None
    // );

    // test mssql connection
    let mut db = Dbs::SqlServer.connect()?;
    let schema = db.schema()?;
    println!("{schema:#?}");

    Ok(())   

}
