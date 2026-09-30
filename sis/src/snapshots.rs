pub mod processes;
pub mod collector;
pub mod timesnap;


pub struct CurrentSnapshot {
    pub timestamp: timesnap::Timestamp,
    pub processes: Vec<collector::ProcessInfo>,
}