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
            //rss,
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
        ) = match fs::read_to_string(proc_path.join("status")) {
            Ok(content) => {
                self.status_buffer = content;
                Self::_parse_status(&self.status_buffer)
            }
            Err(_) => (0, 0, 0, 0, 0, 0, 0),
        };

        self.cmdline_buffer.clear();
        let command_line = match fs::read_to_string(proc_path.join("cmdline")) {
            Ok(content) => {

                self.cmdline_buffer.push_str(&content);
                Self::_parse_command_line(
                    &self.cmdline_buffer
                )
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

        Ok(
            ProcessInfo {
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
                    vm_rss,
                    vm_peak,
                    vm_hwm,
                    vm_data,
                    vm_stk,
                    vm_exe,
                    vm_lib,
                },
                io: IOB {
                    read_bytes,
                    write_bytes,
                    read_syscalls,
                    write_syscalls,
                },
                resources: Resources { num_fds },
            },
        )
        
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
    ) -> io::Result<(
        u32,
        char,
        u32,
        u64,
        u64,
        u64,
        u64,
        i32,
        i32,
        u64,
        u64,
        //u64,
        String,
    )> {

        let closing_paren = stat.rfind(')').ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid stat format",
            )
        })?;

        let name = stat[1..closing_paren].to_owned();

        let rest = &stat[closing_paren + 1..];

        let mut parts = rest.split_whitespace();

        /*
         * Field 3: state
         */
        let state = parts
            .next()
            .and_then(|value| value.chars().next())
            .unwrap_or('?');

        /*
         * Field 4: ppid
         */
        let ppid = parts
            .next()
            .and_then(|value| value.parse::<u32>().ok())
            .unwrap_or(0);

        /*
         * Fields 5..10
         *
         * We don't currently need:
         *
         * pgrp
         * session
         * tty_nr
         * tpgid
         * flags
         * minflt
         * cminflt
         * majflt
         * cmajflt
         *
         * Skip until utime.
         */
        for _ in 0..9 {
            parts.next();
        }

        /*
         * Field 14: utime
         */
        let utime = parts
            .next()
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or(0);

        /*
         * Field 15: stime
         */
        let stime = parts
            .next()
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or(0);

        /*
         * Field 16: cutime
         */
        let cutime = parts
            .next()
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or(0);

        /*
         * Field 17: cstime
         */
        let cstime = parts
            .next()
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or(0);

        /*
         * Field 18: priority
         */
        let priority = parts
            .next()
            .and_then(|value| value.parse::<i32>().ok())
            .unwrap_or(0);

        /*
         * Field 19: nice
         */
        let nice = parts
            .next()
            .and_then(|value| value.parse::<i32>().ok())
            .unwrap_or(0);

        /*
         * Field 20: num_threads
         */
        let thread_count = parts
            .next()
            .and_then(|value| value.parse::<u32>().ok())
            .unwrap_or(0);

        /*
         * Field 21: itrealvalue
         */
        parts.next();

        /*
         * Field 22: starttime
         */
        let start_time = parts
            .next()
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or(0);

        /*
         * Field 23: vsize
         */
        let vsize = parts
            .next()
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or(0);

        /*
         * Field 24: rss
         */
        let rss = parts
            .next()
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or(0);

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
            //rss,
            name,
        ))
    }
 
    /// proc/[pid]/status 
    fn _parse_status(status: &str) -> (u64, u64, u64, u64, u64, u64, u64) {

        let mut vm_peak = 0u64;
        let mut vm_hwm = 0u64;
        let mut vm_data = 0u64;
        let mut vm_stk = 0u64;
        let mut vm_exe = 0u64;
        let mut vm_lib = 0u64;
        let mut vm_rss = 0u64;

        let mut found = 0u8; 
 
        for line in status.lines() {
            if !line.starts_with("Vm") {
                continue;
            }

            let mut parts = line.split_whitespace();

            let Some(field) = parts.next() else {
                continue;
            };

            let Some(value) = parts.next() else {
                continue;
            };

            match field {

                    "VmPeak:" => {
                        if let Ok(value) = value.parse::<u64>() {
                            vm_peak = value;
                            found += 1;
                        }
                    }

                    "VmHWM:" => {
                        if let Ok(value) = value.parse::<u64>() {
                            vm_hwm = value;
                            found += 1;
                        }
                    }

                    "VmData:" => {
                        if let Ok(value) = value.parse::<u64>() {
                            vm_data = value;
                            found += 1;
                        }
                    }

                    "VmStk:" => {
                        if let Ok(value) = value.parse::<u64>() {
                            vm_stk = value;
                            found += 1;
                        }
                    }

                    "VmExe:" => {
                        if let Ok(value) = value.parse::<u64>() {
                            vm_exe = value;
                            found += 1;
                        }
                    }

                    "VmLib:" => {
                        if let Ok(value) = value.parse::<u64>() {
                            vm_lib = value;
                            found += 1;
                        }
                    }

                    "VmRSS:" => {
                        if let Ok(value) = value.parse::<u64>() {
                            vm_rss = value;
                            found += 1;
                        }
                    }

                    _ => {}
            }

            if found == 7 {
                break;
            }

        }

        (
            vm_peak,
            vm_hwm,
            vm_data,
            vm_stk,
            vm_exe,
            vm_lib,
            vm_rss
        )
    }
 
    /// /proc/[pid]/io
    fn _parse_io(io: &str) -> (u64, u64, u64, u64) {

        let mut read_bytes = 0u64;
        let mut write_bytes = 0u64;
        let mut read_syscalls = 0u64;
        let mut write_syscalls = 0u64;
 

        for line in io.lines() {

            let mut parts = line.split_whitespace();

            let Some(field) = parts.next() else {
                continue;
            };

            let Some(value) = parts.next() else {
                continue;
            };

            let Ok(value) = value.parse::<u64>() else {
                continue;
            };

            match field {

                "read_bytes:" => {
                    read_bytes = value;
                }

                "write_bytes:" => {
                    write_bytes = value;
                }

                "syscr:" => {
                    read_syscalls = value;
                }

                "syscw:" => {
                    write_syscalls = value;
                }

                _ => {}
            }
        }

        (
            read_bytes, 
            write_bytes, 
            read_syscalls, 
            write_syscalls
        )
    }
 
     fn _parse_command_line(command_line: &str) -> String {

        let mut result = String::with_capacity(
            command_line.len()
        );

        for byte in command_line.bytes() {

            if byte == b'\0' {
                result.push(' ');
            } else {
                result.push(byte as char);
            }
        }

        result.trim_end().to_owned()
    }

    fn _count_fds(proc_path: &Path) -> u32 {
        
        fs::read_dir(proc_path.join("fd"))
            .map(|entries| entries.count() as u32)
            .unwrap_or(0)
    }


}

