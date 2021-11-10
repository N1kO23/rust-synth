
use std::path::Path;
use std::fs::File;
use hound::{WavWriter, WavSpec};
use std::io::Error;
use std::thread;
use std::time::SystemTime;
use rand::prelude::*;
use rand::Rng;

mod waves;
mod helpers;

fn main() {
    println!("Hello, world!");
    let start = SystemTime::now();
    let waves = helpers::unison(16, waves::sawtooth_wave, 0.75, 100.00, 0.0, 44100.0);

    println!("{} ns", SystemTime::now().duration_since(start).unwrap().as_nanos());

    //waves.push(white_noise(0.25, 44100.0));
    let combined = helpers::mix_waves(&waves);
    make_wav(&combined, "combined.wav");
}

// Make a function that turns a vec<f32> into audio file
fn make_wav(samples: &Vec<f32>, filename: &str) {
    let path = Path::new(filename);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(_why) => panic!("couldn't create {}",
                           display),
        Ok(file) => file,
    };
    
    let spec = WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    
    let mut wav = WavWriter::new(file, spec).unwrap();
    for sample in samples {
        wav.write_sample(*sample).unwrap();
    }
}