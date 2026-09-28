pub mod snapshots;

use std::time::Instant;
use snapshots::processes::ProcessCollector;

fn main() -> std::io::Result<()> {


    println!("Iniciando leitura processos");

    let mut collector = ProcessCollector::new();

    loop {
        
        let start = Instant::now();

        let processes = collector.collect()?;

        let elapsed = start.elapsed();
        
            //println!("Processos: {} | Captura: {:?}",processes.len(),elapsed);
            //println!("Processos coletados: {:?}", processes);
    }

}
