pub mod processes;
pub mod collector;
pub mod timesnap;

use std::ops::Sub;

use crate::snapshots::processes::{
    Entity,
    Execution,
    CPU,
    Memory,
    IOB,
    Resources,
    KeyProcess
};

#[derive(Debug, Clone)]
pub struct ProcessInfo {

    pub entity          : Entity,
    pub execution       : Execution,
    pub cpu             : CPU,
    pub memory          : Memory,
    pub io              : IOB,
    pub resources       : Resources,
    pub keyprocess      : KeyProcess,
}


#[derive(Debug, Clone)]

pub struct Snapshot {
    pub capture_start: timesnap::Timestamp,
    pub capture_finished: timesnap::Timestamp,
    pub processes: Vec<ProcessInfo>,
}

#[derive(Debug, Clone)]
pub struct CaptureProcesses {
    collector: collector::ProcessCollector,
    snapshots_buffer: Vec<Snapshot>,
}

impl CaptureProcesses {
    pub fn new() -> Self {

        CaptureProcesses { 
            snapshots_buffer    : Vec::with_capacity(512), 
            collector           : collector::ProcessCollector::new()
        }
    }

    pub fn capture(&mut self) -> std::io::Result<()> {
        
        let mut processes = Vec::with_capacity(512);

        let capt_start = timesnap::Timestamp::now();
        self.collector.collect(&mut processes)?;
        let capt_finished = timesnap::Timestamp::now();
        
        let capture_duration = capt_finished.duration_since(&capt_start);
        println!(
            "#processos: {} | captura: {:?}",
            processes.len(),
            capture_duration
        );


        let snapshot = Snapshot {
            capture_start       : capt_start,
            capture_finished    : capt_finished,
            processes           : processes,
        };
        self.snapshots_buffer.push(snapshot);

        //println!("Exemplo snapshot: {:?}", self.snapshots_buffer.last());

        Ok(())
    }
}