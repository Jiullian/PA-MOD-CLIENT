use std::net::IpAddr;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::thread::sleep;
use std::time::Duration;
use tokio::sync::mpsc;

pub const IPV4_HEADER_LEN: usize = 20;
pub const ETHERNET_HEADER_LEN: usize = 14;

pub struct Config {
    interface_ip: IpAddr,
    source_port: u16,
    destination_ip: IpAddr,
    ports_to_scan: Vec<u16>,
    wait_after_send: Duration,
    timeout: Duration,
    all_sent: Arc<AtomicBool>,
}


fn tcp_packet(target: String){




}

pub(crate) async fn ddos_ping(rx: &mut mpsc::Receiver<()>, target: String, rate_limit: u64) {

        let mut cmp = 0;

        loop {
            tokio::select! {
            _ = sleep(Duration::from_nanos(rate_limit)) => {
                println!("cmp = {}", cmp);
                tcp_packet(target.clone());
                cmp += 1;
            }
            _ = rx.recv() => {
                // Arrêter d'afficher des messages lorsque le signal est reçu
                println!("Arret fin du temps");
                break;
            }
        }
        }

    }


