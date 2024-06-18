use std::error::Error;
use std::net::UdpSocket;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

use pnet::packet::tcp::{MutableTcpPacket, TcpFlags};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::{MutablePacket, Packet};
use pnet::transport::{self, TransportChannelType, TransportProtocol};
use pnet::util::checksum;
use std::net::Ipv4Addr;


async fn send_tcp(server_addr: &str){
    let destination_ip = Ipv4Addr::new(127, 0, 0, 1);
    let destination_port = 4444;

    // Source IP and port
    let source_ip = Ipv4Addr::new(127, 0, 0, 1);

    let source_port = 12345;

    // Create a buffer for the TCP packet
    let mut tcp_buffer = [0u8; 20]; // TCP header is 20 bytes

    // Create a mutable TCP packet
    let mut tcp_packet = MutableTcpPacket::new(&mut tcp_buffer[..]).unwrap();

    // Set the fields for the TCP packet
    tcp_packet.set_source(source_port);
    tcp_packet.set_destination(destination_port);
    tcp_packet.set_sequence(1);
    tcp_packet.set_flags(TcpFlags::SYN);
    tcp_packet.set_window(64240); // Typical window size

    // Calculate the checksum
    let checksum = checksum(tcp_packet.packet(), 1);
    tcp_packet.set_checksum(checksum);

    // Create the transport channel
    let (mut tx, _) = transport::transport_channel(1024, TransportChannelType::Layer4(TransportProtocol::Ipv4(IpNextHeaderProtocols::Tcp)))
        .expect("Failed to create transport channel");

    // Send the packet
    tx.send_to(tcp_packet, std::net::IpAddr::V4(destination_ip)).expect("Failed to send packet");

    println!("SYN packet sent to {}:{}", destination_ip, destination_port);

}

pub(crate) async fn ddos_tcp(rx: &mut mpsc::Receiver<()>, target_clone: String, target_port_clone: String, rate_limit: u64) {


    println!("TCP DDOS NOT IMPLEMENTED YET !");

    let mut cmp = 0;
    /*loop {
        tokio::select! {
            _ = sleep(Duration::from_nanos(rate_limit)) => {
                println!("cmp = {}", cmp);
                send_tcp(&target_clone).await.expect("Erreur lors de l'envoi du message UDP");
                cmp += 1;
            }
            _ = rx.recv() => {
                // Arrêter d'afficher des messages lorsque le signal est reçu
                println!("Arret fin du temps");
                break;
            }
        }
    }*/
}