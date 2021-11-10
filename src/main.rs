use chrono;
use hound::{WavSpec, WavWriter};
use std::fs::File;
use std::path::Path;
mod helpers;
mod waves;
fn main() {
    // sawtooth_wave
    // sine_wave
    // triangle_wave
    // square_wave
    let chord = vec![
        helpers::unison(16, 5.0, waves::square_wave, 25.96, 0.0, 44100.0),
        helpers::unison(16, 5.0, waves::square_wave, 51.91, 0.0, 44100.0),
        helpers::unison(16, 5.0, waves::square_wave, 103.83, 0.0, 44100.0),
        helpers::unison(16, 5.0, waves::square_wave, 130.81, 0.0, 44100.0),
        helpers::unison(16, 5.0, waves::square_wave, 155.56, 0.0, 44100.0),
        helpers::unison(16, 5.0, waves::square_wave, 174.61, 0.0, 44100.0),
        helpers::unison(16, 5.0, waves::square_wave, 196.00, 0.0, 44100.0),
    ];

    let mut waves = vec![];

    for note in chord {
        for voice in note {
            waves.push(voice);
        }
    }

    //waves.push(white_noise(0.25, 44100.0));
    let combined = helpers::mix_waves(&waves);
    make_wav(
        &combined,
        &format!("chord_{:?}.wav", chrono::offset::Local::now().timestamp()),
    );
}

// Make a function that turns a vec<f32> into audio file
fn make_wav(samples: &Vec<f32>, filename: &str) {
    let path = Path::new(filename);
    let display = path.display();

    let file = match File::create(&path) {
        Err(_why) => panic!("couldn't create {}", display),
        Ok(file) => file,
    };
    let spec = WavSpec {
        channels: 2,
        sample_rate: 44100,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut wav = WavWriter::new(file, spec).unwrap();
    for sample in samples {
        // each sample gets written multiple times per channel
        // so on a 2 channel (stereo) wav each sample gets written
        // It should look like this, each sample should be written TWICE
        // one for the left channel and one for the right channel
        // with the difference being the amplitude to determine panning

        // Sample 1   Sample 2   Sample 3   Sample 4   Sample 5...
        // 00         11         22         33         44
        // LR         LR         LR         LR         LR

        // where in our case this for loops fuck with it
        // and it ends up doing the following
        // Sample 1   Sample 2   Sample 3   Sample 4   Sample 5...
        // 01         23         45         67         89
        // LR         LR         LR         LR         LR
        // which is exactly why its almost stereo but also faster
        // cus the samples slightly differ and its twice as fast
        // essentially we need a panner that would run in that for loop and be aware of
        // how much each sample has to pan for each channel and that would multiply the amplitude of the opposing channel(s)

        wav.write_sample(*sample).unwrap();
    }
}
