use std::path::Path;
use chrono::{Local};
use std::io::{self, Write};
use std::fs::{self, OpenOptions};

const LOG_DIR: &str = "logs";

pub struct LineLog {
    pub line: String
}

impl LineLog {
    
    pub fn save_line(&self) -> io::Result<()> {

        fs::create_dir_all(LOG_DIR)?;

        let filename = format!(
            "{}.log", 
            Local::now().format("%Y-%m-%d")
        );

        let path = Path::new(LOG_DIR).join(filename);
        
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;

        file.write(self.line.as_bytes())?;
        //file.sync_data()?;

        Ok(())
    }
}