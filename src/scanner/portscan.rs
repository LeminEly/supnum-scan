use std::net::{IpAddr, SocketAddr, ToSocketAddrs};
use std::time::Duration;
use futures::stream::{FuturesUnordered, StreamExt};
use tokio::net::TcpStream;
use tokio::time::timeout;

pub async fn scan_ports(target: &str) -> Vec<u16> {
    // 🔹 Résolution DNS
    let ip: IpAddr = match (target, 0).to_socket_addrs() {
        Ok(mut addrs) => match addrs.next() {
            Some(addr) => addr.ip(),
            None => return vec![],
        },
        Err(_) => {
            eprintln!("❌ Target invalide");
            return vec![];
        }
    };

    let mut open_ports = Vec::new();
    let mut tasks = FuturesUnordered::new();

    let max_concurrent = 800; // ajuster selon le réseau
    let timeout_duration = Duration::from_millis(600);

    for port in 1u16..=65535 {
        let addr = SocketAddr::new(ip, port);

        tasks.push(async move {
            match timeout(timeout_duration, TcpStream::connect(addr)).await {
                Ok(Ok(_)) => Some(port), // connecté = ouvert
                _ => None,               // timeout / refus = fermé
            }
        });

        // backpressure
        if tasks.len() >= max_concurrent {
            if let Some(res) = tasks.next().await {
                if let Some(p) = res {
                    println!("🟢 Port open: {}", p);
                    open_ports.push(p);
                }
            }
        }
    }

    while let Some(res) = tasks.next().await {
        if let Some(p) = res {
            println!("🟢 Port open: {}", p);
            open_ports.push(p);
        }
    }

    open_ports.sort_unstable();
    open_ports
}
