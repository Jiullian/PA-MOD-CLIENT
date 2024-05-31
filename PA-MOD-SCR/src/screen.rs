use std::collections::HashMap;
use xcap::Window;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct Screen {
    id: u32,
    attack: String,
    output: HashMap<String, String>,
}

fn sanitize_filename(title: &str) -> String {
    let invalid_chars = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];
    title.chars().map(|c| if invalid_chars.contains(&c) { '_' } else { c }).collect()
}

#[cfg(feature = "libraries")]
pub fn window_capture() {
    let windows = match Window::all() {
        Ok(windows) => windows,
        Err(e) => {
            eprintln!("Erreur lors de la récupération des fenêtres : {:?}", e);
            return;
        }
    };

    let mut screen_info = Screen {
        id: 1, // Vous pouvez ajuster cela comme nécessaire
        attack: "screenshot".to_string(),
        output: HashMap::new(),
    };

    for (index, window) in windows.iter().enumerate() {
        let image = match window.capture_image() {
            Ok(img) => img,
            Err(e) => {
                eprintln!("Erreur lors de la capture d'image de la fenêtre : {:?}", e);
                continue;
            }
        };

        let filename = format!("window-{}.png", sanitize_filename(&window.title()));
        match image.save(&filename) {
            Ok(_) => {
                println!("Image sauvegardée : {}", filename);
                screen_info.output.insert(format!("image{}", index + 1), filename);
            },
            Err(e) => eprintln!("Erreur lors de la sauvegarde de l'image : {:?}", e),
        }
    }

    let json = serde_json::to_string(&screen_info).unwrap();
    println!("{}", json);
}

#[cfg(not(feature = "libraries"))]
pub fn window_capture() {
    println!("La fonctionnalité de capture d'écran n'est pas prise en charge sur ce système");
}
