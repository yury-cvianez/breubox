use std::{
    fs,
    io,
    path::Path,
};



#[derive(Debug, Clone)]
pub struct Process {
    pub pid: u32,
    pub name: String,
}

pub struct ProcessCollector{
    processes: Vec<Process>,
}

impl ProcessCollector {

    /// create a new collector with a pre-allocated vector of 256 slots
    /// it is reusable for each collection
    pub fn new() -> Self {
        ProcessCollector { processes: Vec::with_capacity(256) }
    }

    pub fn collect(&mut self) -> io::Result<&[Process]> {
        
        self.processes.clear();

        for entry in fs::read_dir("/proc")? {

            let entry = entry?;
            let path = entry.path();

            let Some(file_name) = path.file_name() else {
                continue;
            };

            let Some(pid_str) = file_name.to_str() else {
                continue;
            };

            let Ok(pid) = pid_str.parse::<u32>() else {
                continue;
            };

            let Ok(name) = Self::_read_process_name(&path) else {
                continue;
            };

            self.processes.push(Process {
                pid,
                name,
            });
        }

        Ok(&self.processes)
    }

    fn _read_process_name(proc_path: &Path) -> io::Result<String> {

        let comm_path = proc_path.join("comm");
        let name = fs::read_to_string(comm_path)?
            .trim()
            .to_string();
        Ok(name)
    } 

}

