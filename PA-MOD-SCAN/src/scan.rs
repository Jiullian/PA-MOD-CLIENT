use std::sync::mpsc::Receiver;
use function_utils::subprocess_run;
use ipnetwork::IpNetwork;
extern crate regex;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use crate::function_utils;
use std::collections::HashSet;

// a copier coller pour le json
#[derive(Serialize, Deserialize)]
struct DataReceived {
    id: u32,
    attack: String,
}
fn receive_data_json_to_str(data: String) -> DataReceived {
    let p = serde_json::from_str::<DataReceived>(&data).expect("Erreur JSON");
    p
}
#[derive(Serialize, Deserialize)]
struct DataSend {
    id: u32,
    attack: String,
    ipup: Vec<String>
}
fn send_json_data(ips:Vec<String>, received:DataReceived){

    let mut data = DataSend {
        id: 0, // Remplacez par la valeur appropriée
        attack: String::new(), // Remplacez par la valeur appropriée
        ipup: Vec::new(),
    };

    data.id = received.id;
    data.attack = received.attack;
    data.ipup = ips;

    let json_string = serde_json::to_string(&data);

    println!("Serialized JSON: {:?}", json_string)
}

// a copier coller pour le json


fn generate_ips(base_ip: &str, subnet_mask: &str) -> Vec<String> {
    let network_str = format!("{}/{}", base_ip, subnet_mask);
    let network = network_str.parse::<IpNetwork>().expect("Invalid network");

    let mut ips = Vec::new();

    for ip in network.iter() {
        ips.push(ip.to_string());
    }
    ips
}


pub(crate) fn find_net() -> Vec<Vec<String>> {
    let tofind = subprocess_run("ipconfig");
    let re = Regex::new(r"(\b25[0-4]|\b2[0-4][0-9]|\b[01]?[0-9][0-9]?)(\.(25[0-5]|2[0-4][0-9]|[01]?[1-9][1-9]?)){3}").unwrap();
    let mut network_set = HashSet::new();

    // Collect unique networks
    for interface_match in re.find_iter(&tofind) {
        let ip = interface_match.as_str();
        // We assume a /24 subnet for simplification
        let network = format!("{}/24", ip);
        network_set.insert(network);
    }

    let mut ips = Vec::new();
    for network_str in network_set {
        let network = network_str.parse::<IpNetwork>().expect("Invalid network");
        let mut network_ips = Vec::new();
        for ip in network.iter() {
            network_ips.push(ip.to_string());
        }
        ips.push(network_ips);
    }

    ips
}

pub(crate) fn scan(ips: Vec<Vec<String>>, ordre: String) {
    let data_json = receive_data_json_to_str(ordre);
    let mut ips_up: Vec<String> = Vec::new();
    let mut scanned_ips = HashSet::new();  // HashSet pour stocker les IPs déjà scannées

    println!("ID du lancement = {}", data_json.id);

    for subnet_ips in ips.iter() {
        for ip in subnet_ips.iter() {
            if !scanned_ips.contains(ip) {  // Vérifier si l'IP a déjà été scannée
                if up_or_not(ip) {
                    println!("{} is UP", ip);
                    ips_up.push(ip.to_string());
                } else {
                    // println!("{} is DOWN", ip);
                }
                scanned_ips.insert(ip.to_string());  // Ajouter l'IP au HashSet
            }
        }
    }
    println!("fin du ID = {}", data_json.id);
    send_json_data(ips_up, data_json);
}



fn up_or_not(ip: &str) -> bool {
    let cmd= format!("ping {} -n 1 -w 1",ip);

    let sortie = subprocess_run(&*cmd);

    if sortie.contains("TTL"){
        true
    }else{
        false
    }

}