pub mod snapshots;

use snapshots::CaptureProcesses;

fn main() -> std::io::Result<()> {

    let mut snaps = CaptureProcesses::new();

    for _ in 0..25 {

        let _ = snaps.capture()?;
    }

    Ok(())
}
