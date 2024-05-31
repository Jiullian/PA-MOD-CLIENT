#[cfg(feature = "libraries")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
#[cfg(feature = "libraries")]
use hound;
#[cfg(feature = "libraries")]
use std::sync::{Arc, Mutex};

use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct RecordInfo {
    id: u32,
    attack: String,
    output: String,
}

#[cfg(feature = "libraries")]
pub fn record() {
    let host = cpal::default_host();
    let device = host.default_input_device().expect("Aucun périphérique d'entrée audio disponible");
    let mut supported_configs_range = device.supported_input_configs()
        .expect("Erreur lors de la récupération des configurations d'entrée audio");
    let supported_config = supported_configs_range.find(|config| {
        matches!(config.sample_format(), cpal::SampleFormat::F32 | cpal::SampleFormat::I16 | cpal::SampleFormat::U16)
    }).expect("no supported config?!")
        .with_max_sample_rate();

    let sample_format = supported_config.sample_format();
    let config = supported_config.config();

    let spec = hound::WavSpec {
        channels: config.channels as u16,
        sample_rate: config.sample_rate.0,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let samples: Arc<Mutex<Vec<i16>>> = Arc::new(Mutex::new(Vec::new()));

    let err_fn = |err| eprintln!("Une erreur est survenue: {}", err);
    let stream = match sample_format {
        cpal::SampleFormat::F32 => {
            let samples = Arc::clone(&samples);
            device.build_input_stream(
                &config,
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    let mut samples = samples.lock().unwrap();
                    for &sample in data {
                        let sample_int = (sample * i16::MAX as f32) as i16;
                        samples.push(sample_int);
                    }
                },
                err_fn,
                None
            )
        },
        cpal::SampleFormat::I16 => {
            let samples = Arc::clone(&samples);
            device.build_input_stream(
                &config,
                move |data: &[i16], _: &cpal::InputCallbackInfo| {
                    let mut samples = samples.lock().unwrap();
                    samples.extend_from_slice(data);
                },
                err_fn,
                None
            )
        },
        cpal::SampleFormat::U16 => {
            let samples = Arc::clone(&samples);
            device.build_input_stream(
                &config,
                move |data: &[u16], _: &cpal::InputCallbackInfo| {
                    let mut samples = samples.lock().unwrap();
                    for &sample in data {
                        let sample = sample as i16;
                        samples.push(sample);
                    }
                },
                err_fn,
                None
            )
        },
        _ => panic!("Format d'échantillonnage non pris en charge!"),
    }.unwrap();

    stream.play().unwrap();
    std::thread::sleep(std::time::Duration::from_secs(10));
    drop(stream);

    let samples = samples.lock().unwrap();
    let mut writer = hound::WavWriter::create("enregistrement.wav", spec).unwrap();
    for sample in samples.iter() {
        writer.write_sample(*sample).unwrap();
    }
    writer.finalize().unwrap();

    let record_info = RecordInfo {
        id: 1,
        attack: "record".to_string(),
        output: "enregistrement.wav".to_string(),
    };
    let json = serde_json::to_string(&record_info).unwrap();
    println!("{}", json);
}

#[cfg(not(feature = "libraries"))]
pub fn record (){
    println!("La fonctionnalité d'enregistrement audio n'est pas prise en charge sur ce système");
}
