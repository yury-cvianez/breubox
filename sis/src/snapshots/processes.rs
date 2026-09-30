#[derive(Debug, Clone)]
pub struct Entity {

    pub pid: u32,
    pub ppid: u32,
    pub name: String,
    pub start_time: u64,
}

#[derive(Debug, Clone)]
pub struct Execution {

    pub state: char,
    pub thread_count: u32,
    pub command_line: String,
    pub priority: i32,       
    pub nice: i32,           
}

#[derive(Debug, Clone)]
pub struct CPU {

    pub utime: u64,
    pub stime: u64,
    pub cutime: u64,
    pub cstime: u64,
}

#[derive(Debug, Clone)]
pub struct Memory {

    pub vm_rss: u64,
    pub vm_size: u64,
    pub vm_peak: u64,
    pub vm_hwm: u64,
    pub vm_data: u64,
    pub vm_stk: u64,
    pub vm_exe: u64,
    pub vm_lib: u64,
}

#[derive(Debug, Clone)]
pub struct IOB {

    pub read_bytes: u64,
    pub write_bytes: u64,
    pub read_syscalls: u64,
    pub write_syscalls: u64,
}

#[derive(Debug, Clone)]
pub struct Resources {

    pub num_fds: u32,
}