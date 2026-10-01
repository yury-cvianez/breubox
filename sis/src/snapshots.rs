pub mod processes;
pub mod collector;
pub mod timesnap;


pub struct Snapshot {
    pub key: u64,
    pub timestamp: timesnap::Timestamp,
    pub processes: Vec<collector::ProcessInfo>,
}

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

        
        let start = std::time::Instant::now();
        let processes = self.collector.collect()?;
        let elapsed = start.elapsed();

        println!(
            "#processos: {} | captura: {:?}",
            processes.len(),
            elapsed
        );

        Ok(())
    }
}