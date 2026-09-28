use std::{
    fs, 
    io, 
    path::{Path, PathBuf},
};

use crate::snapshots::processes::{
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

    processes       : Vec<ProcessInfo>,
    stat_buffer     : String,
    status_buffer   : String,
    cmdline_buffer  : String,
    io_buffer       : String,

}

impl ProcessCollector {

    /// create a new collector with a pre-allocated vector of 256 slots
    /// vec and buffer, it is reusable for each collection
    pub fn new() -> Self {
        ProcessCollector { 

            processes       : Vec::with_capacity(256),
            stat_buffer     : String::with_capacity(1024),
            status_buffer   : String::with_capacity(2048),
            cmdline_buffer  : String::with_capacity(1024),
            io_buffer       : String::with_capacity(512),
        }
    }

    pub fn collect(&mut self) -> io::Result<&[ProcessInfo]> {
        
        self.processes.clear();

        for entry in fs::read_dir("/proc")? {
            
            let entry = entry?;
            let path = entry.path();

            let Some(pid) = Self::_get_pid(&path) else {
                continue
            };

            let Ok(process) = self._get_info(&path, pid) else {
                continue;
            };

            self.processes.push(process);
        }

        Ok(&self.processes)
    }

    fn _get_pid(path: &PathBuf) -> Option<u32> {
            
        path.file_name()?
            .to_str()?
            .parse::<u32>()
            .ok()

    }

    fn _get_info(&mut self, proc_path: &Path, pid: u32) -> io::Result<ProcessInfo> {
        
        self.stat_buffer.clear();

        fs::read_to_string(
            proc_path.join("stat")
        ).and_then(|content| {
            self.stat_buffer = content;
            Ok(())
        })?;

        let (
            ppid,
            state,
            thread_count,
            utime,
            stime,
            cutime,
            cstime,
            priority,
            nice,
            start_time,
            vsize,
            rss,
            name,
        ) = Self::_parse_stat(&self.stat_buffer)?;


        self.status_buffer.clear();
        let (
            vm_peak, 
            vm_hwm, 
            vm_data, 
            vm_stk, 
            vm_exe, 
            vm_lib, 
            vm_rss,
            resident_memory,
        ) = match fs::read_to_string(proc_path.join("status")) {
            Ok(content) => {
                self.status_buffer = content;
                Self::_parse_status(&self.status_buffer)
            }
            Err(_) => (0, 0, 0, 0, 0, 0, 0, 0),
        };

        self.cmdline_buffer.clear();
        let command_line = match fs::read_to_string(proc_path.join("cmdline")) {
            Ok(content) => {
                // cmdline tem \0 como separador, substituir por espaço
                content
                    .replace('\0', " ")
                    .trim()
                    .to_string()
            }
            Err(_) => String::new(),
        };

        self.io_buffer.clear();
        let (
            read_bytes, 
            write_bytes, 
            read_syscalls, 
            write_syscalls
        ) = match fs::read_to_string(proc_path.join("io")) {
            Ok(content) => {
                self.io_buffer = content;
                Self::_parse_io(&self.io_buffer)
            }
            Err(_) => (0, 0, 0, 0),
        };

        let num_fds = Self::_count_fds(proc_path);

        Ok(ProcessInfo {
            entity: Entity {
                pid,
                ppid,
                name,
                start_time,
            },
            execution: Execution {
                state,
                thread_count,
                command_line,
                priority,
                nice,
            },
            cpu: CPU {
                utime,
                stime,
                cutime,
                cstime,
            },
            memory: Memory {
                vm_size: vsize,
                vm_rss: rss,
                vm_peak,
                vm_hwm,
                vm_data,
                vm_stk,
                vm_exe,
                vm_lib,
                resident_memory,
            },
            io: IOB {
                read_bytes,
                write_bytes,
                read_syscalls,
                write_syscalls,
            },
            resources: Resources { num_fds },
        })
        
    }

    // fn _read_process_name(proc_path: &Path) -> io::Result<String> {

    //     let comm_path = proc_path.join("comm");
    //     let name = fs::read_to_string(comm_path)?
    //         .trim()
    //         .to_string();
    //     Ok(name)
    // } 

    fn _parse_stat(
        stat: &str,
    ) -> io::Result<(u32, char, u32, u64, u64, u64, u64, i32, i32, u64, u64, u64, String)> {

        let closing_paren = stat.rfind(')').ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "Invalid stat format")
        })?;
 
        let name = stat[1..closing_paren]
            .to_string();
 
        let rest = &stat[closing_paren + 1..];
        let parts: Vec<&str> = rest.split_whitespace().collect();
 
        if parts.len() < 20 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Not enough fields in stat",
            ));
        }
 

        // 0: state
        // 1: ppid
        // 11: utime
        // 12: stime
        // 13: cutime
        // 14: cstime
        // 15: priority
        // 16: nice
        // 17: num_threads
        // 19: starttime
        // 20: vsize
        // 21: rss
 
        let state = parts[0].chars().next().unwrap_or('?');
        let ppid = parts[1].parse().unwrap_or(0);
        let utime = parts[11].parse().unwrap_or(0);
        let stime = parts[12].parse().unwrap_or(0);
        let cutime = parts[13].parse().unwrap_or(0);
        let cstime = parts[14].parse().unwrap_or(0);
        let priority = parts[15].parse().unwrap_or(20);
        let nice = parts[16].parse().unwrap_or(0);
        let thread_count = parts[17].parse().unwrap_or(1);
        let start_time = parts[19].parse().unwrap_or(0);
        let vsize = parts[20].parse().unwrap_or(0);
        let rss = parts[21].parse().unwrap_or(0);
 
        Ok((
            ppid,
            state,
            thread_count,
            utime,
            stime,
            cutime,
            cstime,
            priority,
            nice,
            start_time,
            vsize,
            rss,
            name,
        ))
    }
 
    /// proc/[pid]/status 
    fn _parse_status(status: &str) -> (u64, u64, u64, u64, u64, u64, u64, u64) {

        let mut vm_peak = 0u64;
        let mut vm_hwm = 0u64;
        let mut vm_data = 0u64;
        let mut vm_stk = 0u64;
        let mut vm_exe = 0u64;
        let mut vm_lib = 0u64;
        let mut vm_rss = 0u64;
        let mut resident_memory = 0u64;
 
        for line in status.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 {
                continue;
            }
 
            match parts[0] {
                "VmPeak:" => vm_peak = parts[1].parse().unwrap_or(0),
                "VmHWM:" => vm_hwm = parts[1].parse().unwrap_or(0),
                "VmData:" => vm_data = parts[1].parse().unwrap_or(0),
                "VmStk:" => vm_stk = parts[1].parse().unwrap_or(0),
                "VmExe:" => vm_exe = parts[1].parse().unwrap_or(0),
                "VmLib:" => vm_lib = parts[1].parse().unwrap_or(0),
                "VmRSS:" => vm_rss = parts[1].parse().unwrap_or(0),
                _ => {}
            }
        }
        
        resident_memory = vm_rss;

        (vm_peak, vm_hwm, vm_data, vm_stk, vm_exe, vm_lib, vm_rss, resident_memory)
    }
 
    /// /proc/[pid]/io
    fn _parse_io(io: &str) -> (u64, u64, u64, u64) {

        let mut read_bytes = 0u64;
        let mut write_bytes = 0u64;
        let mut read_syscalls = 0u64;
        let mut write_syscalls = 0u64;
 
        for line in io.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 {
                continue;
            }
 
            match parts[0] {
                "read_bytes:" => read_bytes = parts[1].parse().unwrap_or(0),
                "write_bytes:" => write_bytes = parts[1].parse().unwrap_or(0),
                "syscr:" => read_syscalls = parts[1].parse().unwrap_or(0),
                "syscw:" => write_syscalls = parts[1].parse().unwrap_or(0),
                _ => {}
            }
        }
 
        (read_bytes, write_bytes, read_syscalls, write_syscalls)
    }
 
    fn _count_fds(proc_path: &Path) -> u32 {
        
        fs::read_dir(proc_path.join("fd"))
            .map(|entries| entries.count() as u32)
            .unwrap_or(0)
    }


}

