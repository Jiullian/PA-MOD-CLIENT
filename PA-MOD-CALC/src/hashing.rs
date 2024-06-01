use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::thread::sleep;
use hex::ToHex;
use md5;
use sha1::{Digest, Sha1};

pub async fn brute_wordlist(hash: &str, wordlist_nb: &str, type_of_hash: &str) -> Result<Option<String>, Box<dyn Error>> {
    let wordlist_path = format!("bibli/rck{}.txt", wordlist_nb);
    let type_of_hash = type_of_hash.to_lowercase();

    match type_of_hash.as_str() {
        "md5" => {
            return brute_md5(hash, &wordlist_path).await;
        }
        "sha1" => {
            return brute_sha1(hash, &wordlist_path).await;
        }
        _ => {
            return Err("Type de hash non supporté".into());
        }
    }
}

async fn brute_md5(hash: &str, wordlist_path: &str) -> Result<Option<String>, Box<dyn Error>> {
    if let Ok(lines) = read_lines(wordlist_path) {
        for line in lines.lines() {
            if let Ok(password) = line {
                let digest = format!("{:x}", md5::compute(password.as_bytes()));
                if digest == hash {
                    return Ok(Some(password));
                }
            }
        }
    }
    Ok(None)
}

async fn brute_sha1(hash: &str, wordlist_path: &str) -> Result<Option<String>, Box<dyn Error>> {
    if let Ok(lines) = read_lines(wordlist_path) {
        for line in lines.lines() {
            if let Ok(password) = line {
                let mut hasher = Sha1::new();
                hasher.update(password.as_bytes());
                let result = hasher.finalize();
                let digest_hex = hex::encode(result);

                if digest_hex == hash {
                    return Ok(Some(password));
                }
            }
        }
    }
    Ok(None)
}

fn read_lines<P>(filename: P) -> io::Result<io::BufReader<File>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file))
}
