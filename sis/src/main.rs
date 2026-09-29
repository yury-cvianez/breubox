pub mod snapshots;

use std::time::Instant;
use snapshots::collector::ProcessCollector;

fn main() -> std::io::Result<()> {


    println!("Iniciando leitura processos");

    let mut collector = ProcessCollector::new();

    for i in 0..10 {
        let start = Instant::now();

        let processes = collector.collect()?;

        let elapsed = start.elapsed();

        println!(
            "#{:02} | processos: {} | captura: {:?}",
            i + 1,
            processes.len(),
            elapsed
        );
    }

    Ok(())

    // loop {
        
    //     let start = Instant::now();

    //     let processes = collector.collect()?;

    //     let elapsed = start.elapsed();
        
    //     println!("Processos: {} | Captura: {:?}",processes.len(),elapsed);
    //     println!("Processos coletados: {:?}", processes);
    // }

}
