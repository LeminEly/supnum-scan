use std::fs::{self, File, OpenOptions};
use std::io::{Write, Result};
use std::path::Path;

fn next_filename(target: &str) -> String {
    let safe_target = target.replace(":", "_").replace("/", "_");
    let dir = Path::new("scans");

    if !dir.exists() {
        fs::create_dir(dir).expect("Impossible de créer le dossier scans/");
    }

    let mut index = 1;
    loop {
        let filename = format!("scans/{}_{}.txt", safe_target, index);
        if !Path::new(&filename).exists() {
            return filename;
        }
        index += 1;
    }
}

pub fn write_scan(
    target: &str,
    open_ports: &[u16],
    duration_ms: u128,
) -> Result<String> {
    let filename = next_filename(target);
    let mut file = File::create(&filename)?;

    writeln!(file, "SupNum Scan Report")?;
    writeln!(file, "==================")?;
    writeln!(file, "Target   : {}", target)?;
    writeln!(file, "Ports    : {}", open_ports.len())?;
    writeln!(file, "Duration : {} ms", duration_ms)?;
    writeln!(file)?;
    writeln!(file, "Open ports:")?;

    for port in open_ports {
        writeln!(file, "  - {}", port)?;
    }

    Ok(filename)
}

pub fn append_nmap(path: &str, nmap_output: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .open(path)?;

    writeln!(file)?;
    writeln!(file, "======================")?;
    writeln!(file, "NMAP ADVANCED SCAN")?;
    writeln!(file, "======================")?;
    writeln!(file)?;
    writeln!(file, "{}", nmap_output)?;

    Ok(())
}