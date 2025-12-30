mod banner;
mod scanner;
mod args;
mod output;

use std::io::{self, Write};
use std::time::Instant;

#[tokio::main]
async fn main() {
    banner::show("Mohamed Lemine Ely", "https://github.com/LeminEly");

    let target = args::get_target();
    println!("🎯 Target: {}\n", target);

    let start = Instant::now();

    let open_ports = scanner::portscan::scan_ports(&target).await;

    let duration = start.elapsed().as_millis();

    println!("\n📌 Open ports: {:?}", open_ports);
    println!("⏱️  Scan time: {} ms", duration);

    
    let report_path = match output::write_scan(&target, &open_ports, duration) {
    Ok(path) => {
        println!("📄 Rapport sauvegardé : {}", path);
        path
    }
    Err(e) => {
        eprintln!("❌ Erreur écriture fichier : {}", e);
        return;
    }
};

    if open_ports.is_empty() {
        return;
    }

    print!("\n❓ Continuer avec nmap ? (yes/no): ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    if choice.trim().eq_ignore_ascii_case("yes") {
    let ports_str = open_ports
        .iter()
        .map(|p| p.to_string())
        .collect::<Vec<_>>()
        .join(",");

   
    println!("\n🚀 Scan avancé avec nmap");
    println!(
        "👉 Commande : nmap -sV -A -p {} {}",
        ports_str, target
    );
    println!("──────────────────────────────────────────────────────────");

   
    std::io::stdout().flush().unwrap();

    
    match scanner::nmap::nmap_scan(&target, &open_ports) {
        Ok(nmap_output) => {
            println!("{}", nmap_output);

            if let Err(e) = output::append_nmap(
                &report_path,
                &format!(
                    "Commande : nmap -sV -A -p {} {}\n\n{}",
                    ports_str, target, nmap_output
                ),
            ) {
                eprintln!("❌ Impossible d'écrire nmap dans le rapport: {}", e);
            } else {
                println!("📄 Résultat nmap ajouté au rapport");
            }
        }
        Err(e) => eprintln!("❌ {}", e),
    }
}

 else {
        println!("✅ Scan terminé.");
    }
}
