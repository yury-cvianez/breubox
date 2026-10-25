/*

CD01    - SqlServer connection
CD02    - SqlServer operation

*/

pub mod write;

use chrono::{Utc};

use crate::aud::write::LineLog;


pub struct AuditDetails {
    
    who     : String,
    status  : String,
    message : Option<String>,
}
struct AuditRegister { audit: AuditDetails }



impl AuditDetails {

    pub fn new(
        who     : String, 
        status  : String, 
        message : Option<String>
    ) -> std::io::Result<()>{
        
        let audit = AuditDetails {
            who,
            status,
            message
        };

        AuditRegister { audit }.record()
    }

}

impl AuditRegister {

    fn record(&self) -> std::io::Result<()> {
        
        let ts = Utc::now().timestamp();
        let line = self._format(ts);

        LineLog{line}.save_line().unwrap_or_else(|e| {
            eprintln!("Failed to save audit log: {}", e);
        });

        Ok(())

    }

    fn _format(&self, ts: i64) -> String {
        
       let mut line = format!(  
            "{}|{}|{}",
            ts,
            self._clean(&self.audit.who),
            self._clean(&self.audit.status)
        );

        if let Some(msg) = &self.audit.message {
            line.push('|');
            line.push_str(&self._clean(msg));
        }

        line.push('\n');

        line

    }
    
    fn _clean(&self, s : &str) -> String {

        s.replace(['|', '\n', '\r'], " ")
    }
}
