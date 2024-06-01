mod hashing;
mod utils;

use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        0 | 1 => {
            println!("Il manque le hash");
            return;
        }
         4 => {} // Tout va bien, continue
        _ => {
            println!("erreur arguments");
            return;
        }
    }

    let ip_srv = "51.77.193.65";
    let port_srv = "8080";

    if let Err(e) = utils::get_rockyou(ip_srv, port_srv).await {
        eprintln!("Erreur lors du téléchargement de rockyou: {}", e);
        return;
    }

    let nb_wordlist = &args[1];
    let primitive = &args[2];
    let hash = &args[3];

    println!("HASH loaded -> {} <-", hash);

    match hashing::brute_wordlist(hash,nb_wordlist,primitive).await {
        Ok(Some(password)) => println!("Mot de passe trouvé: {}", password),
        Ok(None) => println!("Mot de passe non trouvé"),
        Err(e) => eprintln!("Une erreur s'est produite: {}", e),
    }
}
