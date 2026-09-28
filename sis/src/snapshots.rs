pub mod processes;
pub mod inf_pid;

use crate::time::Timestamp;

pub struct CurrentSnapshot{
    timestamp: Timestamp,
    processes: Vec<processes::ProcessInfo>
}