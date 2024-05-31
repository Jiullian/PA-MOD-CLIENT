use std::net::{TcpStream, SocketAddr};
use std::time::Duration;
use std::io::{self, Read, Write};
use serde::{Serialize, Deserialize};
use serde_json;

// Structure pour les résultats des scans de ports
#[derive(Serialize, Deserialize)]
struct PortScanResult {
    ip: String,
    port: u16,
    status: String,
    banner: String,
}

fn get_banner(ip: &str, port: u16) -> io::Result<String> {
    let address = format!("{}:{}", ip, port);
    let socket_addr = address.parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    let mut stream = TcpStream::connect_timeout(&socket_addr, Duration::from_millis(200))?;
    // Sending a basic probe to elicit a response
    let _ = stream.write_all(b"Hello\r\n");

    let mut response = [0; 1024]; // buffer for the response
    let nbytes = stream.read(&mut response)?;
    if nbytes == 0 {
        return Ok("No response".to_string());
    }
    Ok(String::from_utf8_lossy(&response[..nbytes]).to_string())
}

pub fn scan_ports(ip: &str, advanced: bool) {
    let ports = 1..=65535;
    let mut results = Vec::new();

    for port in ports {
        if port == 135 { // Ignorer le port 135
            continue;
        }

        let address = format!("{}:{}", ip, port);
        let status = match TcpStream::connect_timeout(&address.parse().unwrap(), Duration::from_millis(100)) {
            Ok(_) => "open",
            Err(_) => "closed",
        };

        let banner = if advanced && status == "open" {
            get_banner(ip, port).unwrap_or_else(|_| "Failed to retrieve banner".to_string())
        } else {
            String::new()
        };

        results.push(PortScanResult {
            ip: ip.to_string(),
            port,
            status: status.to_string(),
            banner,
        });
    }

    let json_results = serde_json::to_string(&results).unwrap();
    println!("Scan results: {}", json_results);
}

pub fn scan_port_range(ip: &str, start_port: u16, end_port: u16, advanced: bool) {
    let mut results = Vec::new();

    for port in start_port..=end_port {
        if port == 135 { // Ignorer le port 135
            continue;
        }

        let address = format!("{}:{}", ip, port);
        let socket_addr: SocketAddr = address.parse().unwrap();
        let status = match TcpStream::connect_timeout(&socket_addr, Duration::from_millis(100)) {
            Ok(_) => "open",
            Err(_) => "closed",
        };

        if status == "open" {  // Ajouter le résultat uniquement si le port est ouvert
            let banner = if advanced {
                get_banner(ip, port).unwrap_or_else(|_| "Failed to retrieve banner".to_string())
            } else {
                String::new()
            };

            results.push(PortScanResult {
                ip: ip.to_string(),
                port,
                status: status.to_string(),
                banner,
            });
        }
    }

    if results.is_empty() {
        println!("No open ports found for IP: {}", ip);
    } else {
        let json_results = serde_json::to_string(&results).unwrap();
        println!("Open ports: {}", json_results);
    }
}
