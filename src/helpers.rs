use rand::Rng;

use crate::waves;

pub fn unison(voices: i8, wave_function: waves::WaveFunction, amplitude: f32, frequency: f32, phase: f32, sample_rate: f32) -> Vec<Vec<f32>> {
  let mut unison_waves = vec![];
  for _ in 0..voices {
    let mut rng = rand::thread_rng();
    unison_waves.push(wave_function(amplitude, frequency + (rand::random::<f32>() - 0.5) * 0.1, rng.gen_range(0.0..360.0), sample_rate));
  }
  return unison_waves;
}

pub fn mix_waves(waves: &Vec<Vec<f32>>) -> Vec<f32> {
  let mut result = Vec::new();
  for i in 0..waves[0].len() {
      let mut sum = 0.0;
      for j in 0..waves.len() {
          sum += waves[j][i];
      }
      result.push(sum);
  }
  result
}