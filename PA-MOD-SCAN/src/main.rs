mod scan;
mod function_utils;
mod port;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let advanced_mode = args.contains(&"--advanced".to_string());

    if args.len() > 2 {
        if args[1] == "--ip" {
            let ip = &args[2];
            if args.len() > 4 && args[3] == "--port1" {
                let port1 = args[4].parse::<u16>().expect("Invalid port number");
                if args.len() > 6 && args[5] == "--port2" {
                    let port2 = args[6].parse::<u16>().expect("Invalid port number");
                    println!("Scanning ports {} to {} for IP: {}", port1, port2, ip);
                    port::scan_port_range(ip, port1, port2, advanced_mode);
                } else {
                    println!("Scanning port {} for IP: {}", port1, ip);
                    port::scan_port_range(ip, port1, port1, advanced_mode);
                }
            } else {
                println!("No port information provided, scanning all ports for IP: {}", ip);
                port::scan_ports(ip, advanced_mode);
            }
        }
    } else {
        let networks = scan::find_net();
        let ordre_json = r#"{"id": 1, "attack": "Test Attack"}"#.to_string();
        scan::scan(networks, ordre_json);
    }

}
