pub static PI: f32 = std::f64::consts::PI as f32;

pub type WaveFunction = fn(frequency: f32, start_phase: f32, sample_rate: f32) -> Vec<f32>;

pub fn white_noise(sample_rate: f32) -> Vec<f32> {
  let mut samples = Vec::new();
  for _ in 0..(sample_rate as usize) {
    samples.push(rand::random::<f32>() - 0.5);
  }
  return samples;
}

pub fn sawtooth_wave(frequency: f32, start_phase: f32, sample_rate: f32) -> Vec<f32> {
  let mut samples: Vec<f32> = Vec::new();
  for i in 0..(sample_rate as u32) {
    let size = sample_rate / frequency;
    let pos = i as f32 % size;
    samples.push((start_phase + (pos / size)) % 1.0 * 0.5);
  }
  return samples;
}

pub fn sine_wave(frequency: f32, start_phase: f32, sample_rate: f32) -> Vec<f32> {
  let mut samples: Vec<f32> = Vec::new();
  for i in 0..(sample_rate as u32) {
    let size = sample_rate / frequency;
    let pos = i as f32 % size;
    samples.push(((start_phase + pos / size) * (2.0 * PI)).sin());
  }
  return samples;
}

pub fn triangle_wave(frequency: f32, start_phase: f32, sample_rate: f32) -> Vec<f32> {
  let mut samples: Vec<f32> = Vec::new();
  for i in 0..(sample_rate as u32) {
    let size = sample_rate / frequency;
    let pos = i as f32 % size;
    samples.push((((start_phase + (pos / size)) % 1.0 * 2.0 - 1.0) - 0.5).abs());
  }
  return samples;
}

pub fn square_wave(frequency: f32, start_phase: f32, sample_rate: f32) -> Vec<f32> {
  let mut samples: Vec<f32> = Vec::new();
  for i in 0..(sample_rate as u32) {
    let size = sample_rate / frequency;
    let pos = i as f32 % size;
    samples.push(if start_phase + (pos / size) % 1.0 < 0.5 {
      -1.0
    } else {
      1.0
    });
  }
  return samples;
}
