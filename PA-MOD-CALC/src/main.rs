mod hashing;
mod utils;

use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.iter().count() >  2 {
        println!("Trop d'argument");
        return;
    }

    if args.iter().count() <  2 {
        println!("IL manque le hash");
        return;
    }

    let ip_srv = "51.77.193.65";
    let port_srv = "8080";

    utils::get_rockyou(ip_srv, port_srv).await.expect("error : dl rockyou");


    let hash = args[1].to_string();
    println!("{}",hash);

    match hashing::brute_force_md5_from_wordlist(args[1].as_str()).await {
        Ok(Some(password)) => println!("Password found: {}", password),
        Ok(None) => println!("Password not found"),
        Err(e) => eprintln!("An error occurred: {}", e),
    }


}