#[cfg(feature = "libraries_windows")]
use scrap::{Capturer, Display};
#[cfg(feature = "libraries_windows")]
use std::io::ErrorKind::WouldBlock;
#[cfg(feature = "libraries_windows")]
use std::thread;
#[cfg(feature = "libraries_windows")]
use std::time::Duration;
#[cfg(feature = "libraries_linux")]
use xcap::Monitor;

use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct CaptureInfo {
    id: u32,
    attack: String,
    output: String,
}

#[cfg(target_os = "libraries_linux")]
#[cfg(feature = "libraries")]
pub fn monitor_capture() -> Result<(), Box<dyn std::error::Error>> {
    let monitors = Monitor::all()?;
    for monitor in monitors {
        let image = monitor.capture_image()?;
        let filename = format!("monitor-{}.png", monitor.name());
        image.save(&filename)?;

        let capture_info = CaptureInfo {
            id: 1,
            attack: "screenshot".to_string(),
            output: filename,
        };
        let json = serde_json::to_string(&capture_info).unwrap();
        println!("{}", json);
    }

    Ok(())
}

#[cfg(target_os = "windows")]
#[cfg(feature = "libraries_windows")]
pub fn monitor_capture() -> Result<(), Box<dyn std::error::Error>> {
    let filename = "capture.png";
    let display = Display::primary()?;
    let (width, height) = (display.width(), display.height());
    let mut capturer = Capturer::new(display)?;

    loop {
        match capturer.frame() {
            Ok(frame) => {
                let buffer = frame.to_vec();
                image::save_buffer(filename, &buffer, width as u32, height as u32, image::ColorType::Rgba8)?;
                break;
            }
            Err(error) => {
                if error.kind() == WouldBlock {
                    thread::sleep(Duration::from_millis(100));
                    continue;
                } else {
                    return Err(Box::new(error));
                }
            }
        }
    }

    let capture_info = CaptureInfo {
        id: 1,
        attack: "monitor".to_string(),
        output: filename.to_string(),
    };
    let json = serde_json::to_string(&capture_info).unwrap();
    println!("{}", json);

    Ok(())
}

#[cfg(not(feature = "libraries_linux"))]
#[cfg(not(feature = "libraries_windows"))]
pub fn monitor_capture(){
    println!("La fonctionnalité de capture d'écran n'est pas prise en charge sur ce système");
}