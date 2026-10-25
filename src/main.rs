
pub mod aud;
pub mod mndb;

use crate::aud::AuditDetails;

fn main() -> std::io::Result<()> {

    AuditDetails::new(
        "CD01".into(), 
        
        "ok".into(), 
        None
    ).record()?;

    Ok(())    
}
