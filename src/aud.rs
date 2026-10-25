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

impl AuditDetails {

    pub fn new(who: String, status: String, message: Option<String>) -> Self {

        Self {
            who,
            status,
            message
        }

    }

    pub fn record(&self) -> std::io::Result<()> {
        
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
            self._clean(&self.who),
            self._clean(&self.status)
        );

        if let Some(msg) = &self.message {
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
