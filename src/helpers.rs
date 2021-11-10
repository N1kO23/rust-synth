use crate::waves;

pub fn unison(
  voices: i8,
  detune_amount: f32,
  wave_function: waves::WaveFunction,
  frequency: f32,
  phase: f32,
  sample_rate: f32,
) -> Vec<Vec<f32>> {
  let mut unison_waves = vec![];
  for _ in 0..voices {
    unison_waves.push(wave_function(
      1.0 / voices as f32,
      frequency + (rand::random::<f32>() - 0.5) * detune_amount,
      phase + (rand::random::<f32>()),
      sample_rate,
    ));
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

pub fn panner(wave: &Vec<f32>, angle: f32) -> Vec<f32> {
  let left = (2.0 as f32).sqrt() / 2.0 * (angle.cos() - angle.sin()) * angle;
  let right = (2.0 as f32).sqrt() / 2.0 * (angle.cos() + angle.sin()) * angle;

  return vec![left, right];
}
