use tokio;
use std::error::Error;
use std::net::UdpSocket;

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

#[tokio::main]
async fn main() {




    let mut cmp = 0;

    while cmp != 4 {
        println!("cmp = {}", cmp);
        let thread = tokio::spawn(async{
            send_udp_message("127.0.0.1:4444").await.expect("TODO: panic message");
        });
        cmp += 1;
    }

}
