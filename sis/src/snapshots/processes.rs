use std::{
    fs, io, path::{Path, PathBuf},
};

use crate::snapshots::inf_pid::{
    Entity,
    Execution,
    CPU,
    Memory,
    IOB,
    Resources,
};

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub entity      : Entity,
    pub execution   : Execution,
    pub cpu         : CPU,
    pub memory      : Memory,
    pub io          : IOB,
    pub resources   : Resources,
}

pub struct ProcessCollector{
    processes: Vec<ProcessInfo>,
}

impl ProcessCollector {

    /// create a new collector with a pre-allocated vector of 256 slots
    /// it is reusable for each collection
    pub fn new() -> Self {
        ProcessCollector { processes: Vec::with_capacity(256) }
    }

    pub fn collect(&mut self) -> io::Result<&[ProcessInfo]> {
        
        self.processes.clear();

        for entry in fs::read_dir("/proc")? {
            
            let entry = entry?;
            let path = entry.path();

            let Some(pid) = Self::_get_pid(&path) else {
                continue
            };

            let Ok(process) = Self::_get_info(&path, pid) else {
                continue;
            };

            self.processes.push(process);
        }

        Ok(&self.processes)
    }

    fn _get_pid(path: &PathBuf) -> Option<u32> {
            
        let Some(file_name) = path.file_name() else {
            return None;
        };

        let Some(pid_str) = file_name.to_str() else {
            return None;
        };

        pid_str.parse::<u32>().ok()
    }

    fn _get_info(path: &Path, pid: u32) -> io::Result<ProcessInfo> {
            // let Ok(name) = Self::_read_process_name(&path) else {
            //         continue;
            // };
        let entity = ...;
        let execution = ...;
        let cpu = ...;
        let memory = ...;
        let io = ...;
        let resources = ...;

        Ok(ProcessInfo {
            entity,
            execution,
            cpu,
            memory,
            io,
            resources,
        })
        
    }

    fn _read_process_name(proc_path: &Path) -> io::Result<String> {

        let comm_path = proc_path.join("comm");
        let name = fs::read_to_string(comm_path)?
            .trim()
            .to_string();
        Ok(name)
    } 

}

