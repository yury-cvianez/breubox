pub mod snapshots;

use snapshots::CaptureProcesses;

fn main() -> std::io::Result<()> {

    println!("Iniciando leitura processos");

    let mut snaps = CaptureProcesses::new();

    for _ in 0..10 {

        let _ = snaps.capture()?;

    }

    Ok(())
}
