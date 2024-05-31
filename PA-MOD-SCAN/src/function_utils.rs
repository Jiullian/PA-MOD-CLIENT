use std::process::Command;

pub(crate) fn subprocess_run(cmd: &str) -> String {

    let output = if cfg!(target_os = "windows") {
        Command::new("powershell")
            .arg(cmd)
            .output()
            .expect("Erreur lors de l'exécution de la commande.")
    } else {
        Command::new("bash")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("Erreur lors de l'exécution de la commande.")

    };
    //return
    //println!("{}", String::from_utf8_lossy(&output.stdout).to_string());
    String::from_utf8_lossy(&output.stdout).to_string()
}