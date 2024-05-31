mod record;

use clap::{Arg, Command};

fn main() {
    let matches = Command::new("Audio Recorder")
        .version("1.0")
        .author("Auteur")
        .about("Enregistre l'audio à partir du périphérique d'entrée par défaut")
        .arg(Arg::new("time")
            .long("time")
            .takes_value(true)
            .help("Durée de l'enregistrement en secondes"))
        .get_matches();

    let duration = matches.value_of("time").unwrap_or("10").parse::<u64>().unwrap();
    record::record(duration);
}