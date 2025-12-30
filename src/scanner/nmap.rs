use std::process::Command;

pub fn nmap_scan(target: &str, ports: &[u16]) -> Result<String, String> {
    if ports.is_empty() {
        return Err("Aucun port ouvert, scan nmap ignoré.".into());
    }

    let ports_str = ports
        .iter()
        .map(|p| p.to_string())
        .collect::<Vec<_>>()
        .join(",");

    let output = Command::new("nmap")
        .args(["-sV", "-A", "-p", &ports_str, target])
        .output()
        .map_err(|e| format!("Erreur nmap : {}", e))?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
