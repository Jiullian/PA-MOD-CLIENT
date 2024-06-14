mod ddos_udp;
mod ddos_ping;

use tokio;
use std::error::Error;
use std::env;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};






async fn chrono(tx: mpsc::Sender<()>, ddos_time:u64) {

    sleep(Duration::from_secs(ddos_time)).await;
    println!("chrono over");

    // ESignal de fin
    let _ = tx.send(()).await;
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


    let mut ddos_task;

    match args[4].as_str() {
        "udp" => {
            ddos_task = tokio::spawn(async move {
                ddos_udp::ddos_udp(&mut rx, target).await;
            });

            chrono_task.await.unwrap();
        },
        "tcp" => {
            println!("tcp ddos");
        },
        "ping" => {
            println!("ping ddos");
            ddos_task = tokio::spawn(async move {
                //ddos_udp::ddos_udp(&mut rx, target).await;
            });

            chrono_task.await.unwrap();
        },

        _ => {
            println!("veuillez donner une méthode : tcp, udp ou ping ");
            return ()
        }
    }
}


