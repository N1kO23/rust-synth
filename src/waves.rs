static PI: f32 = std::f64::consts::PI as f32;

pub type WaveFunction = fn(amplitude: f32, frequency: f32, phase: f32, sample_rate: f32) -> Vec<f32>;

pub fn white_noise(amplitude: f32, sample_rate: f32) -> Vec<f32> {
  let mut samples = Vec::new();
  for _ in 0..(sample_rate as usize) {
      samples.push(amplitude * (rand::random::<f32>() - 0.5));
  }
  samples
}

pub fn sawtooth_wave(amplitude: f32, frequency: f32, mut phase: f32, sample_rate: f32) -> Vec<f32> {
  let mut samples: Vec<f32> = Vec::new();
  for _ in 0..sample_rate as u32 {
      samples.push(amplitude * phase);
      phase += frequency / sample_rate;
      if phase > 1.0 {  
          phase -= 1.0; 
      }
  }
  samples
}

pub fn sine_wave(amplitude: f32, frequency: f32, phase: f32, sample_rate: f32) -> Vec<f32> {
  let mut samples: Vec<f32> = Vec::new();
  for i in 0..sample_rate as u32 {
    samples.push((PI * 2.0 * i as f32 / sample_rate * frequency).sin() * amplitude + phase);
  }
  samples
}

pub fn square_wave(amplitude: f32, frequency: f32, phase: f32, sample_rate: f32) -> Vec<f32> {
  let mut samples: Vec<f32> = Vec::new();
  for i in 0..sample_rate as u32 {
      samples.push(
        if (PI * 2.0 * i as f32 / sample_rate * frequency).sin() > 0.0 {
          amplitude
      } else {
          -amplitude
      });
  }
  samples
}

pub fn triangle_wave(amplitude: f32, frequency: f32, phase: f32, sample_rate: f32) -> Vec<f32> {
  let mut samples: Vec<f32> = Vec::new();
  for i in 0..sample_rate as u32 {
      samples.push((2.0 * amplitude / PI) * (2.0 * (PI * i as f32 / sample_rate * frequency).sin().asin()) + phase);
  }
  samples
}