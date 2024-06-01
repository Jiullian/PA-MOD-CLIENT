use std::error::Error;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;
use tokio::task;

pub async fn brute_force_md5_from_wordlist(target_hash: &str) -> Result<Option<String>, Box<dyn Error>> {
    // Convert to bytes
    let target_hash_bytes = hex::decode(target_hash)?;

    // bibli path
    let current_dir = std::env::current_dir()?;
    let bibli_path = current_dir.join("bibli");

    // Perform brute force asynchronously
    task::spawn_blocking(move || {
        brute_force_from_wordlist(&bibli_path, &target_hash_bytes)
    })
        .await.expect("Error while brute forcing")
}

fn brute_force_from_wordlist(bibli_path: &PathBuf, target_hash: &[u8]) -> Result<Option<String>, Box<dyn Error>> {
    for cmp in 1..=15 {
        let file_path = bibli_path.join(format!("rck{}.txt", cmp));

        if let Some(mdp) = brute_force_with_file(&file_path, target_hash)? {
            return Ok(Some(mdp));
        }
    }

    Ok(None)
}

fn brute_force_with_file(file_path: &PathBuf, target_hash: &[u8]) -> Result<Option<String>, Box<dyn Error>> {
    let file = std::fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let word = line?;
        let hash = md5::compute(word.as_bytes());

        if hash.as_slice() == target_hash {
            return Ok(Some(word));
        }
    }

    Ok(None)
}