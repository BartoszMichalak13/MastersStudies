use rand::prelude::*;
use std::f64::consts::PI;

// Classic Monte Carlo for ∫₀¹ √(1 − x²) dx
fn monte_carlo_circle(samples: usize) -> Vec<f64> {
  let mut rng = thread_rng();
  let mut values = Vec::with_capacity(samples);
  for _ in 0..samples {
    let x: f64 = rng.gen_range(0.0..1.0);
    values.push((1.0 - x * x).sqrt());
  }
  values
}

// 2D Monte Carlo (hit-or-miss) for ∫₀¹ √(1 − x²) dx
fn monte_carlo_circle_2_d(samples: usize) -> Vec<f64> {
  let mut rng = thread_rng();
  let mut values = Vec::with_capacity(samples);
  for _ in 0..samples / 2 {
    let x: f64 = rng.gen_range(0.0..1.0);
    let y: f64 = rng.gen_range(0.0..1.0);
    let x_val = (1.0 - x * x).sqrt();
    values.push(if y <= x_val { 1.0 } else { 0.0 });
  }
  values
}

// Stratified Sampling version for ∫₀¹ √(1 − x²) dx
fn stratified_sampling_circle(samples: usize) -> Vec<f64> {
  let mut rng = thread_rng();
  let mut values = Vec::with_capacity(samples);
  let half = (samples as f64 / 2.0).ceil() as usize;

  for _ in 0..half {
    let x: f64 = rng.gen_range(0.0..0.5);
    let y: f64 = rng.gen_range(0.5..1.0);
    let x_val = (1.0 - x * x).sqrt();
    values.push(if y <= x_val { 2.0 } else { 0.0 });
  }

  for _ in half..samples {
    let x: f64 = rng.gen_range(0.5..1.0);
    let y: f64 = rng.gen_range(0.5..1.0);
    let x_val = (1.0 - x * x).sqrt();
    values.push(if y <= x_val { 1.0 } else { 3.0 });
  }
  values
}

// Antithetic Variates version for ∫₀¹ √(1 − x²) dx
fn antithetic_variates_circle(samples: usize) -> Vec<f64> {
  let mut rng = thread_rng();
  let mut values = Vec::with_capacity(samples);
  for _ in 0..samples {
    let u: f64 = rng.gen();
    let x1 = u;
    let x2 = 1.0 - u;
    values.push((1.0 - x1 * x1).sqrt());
    values.push((1.0 - x2 * x2).sqrt());
  }
  values
}

// Helper function to compute mean and variance
fn mean_and_variance(samples: &[f64]) -> (f64, f64) {
  let n = samples.len() as f64;
  let mean = samples.iter().sum::<f64>() / n;
  let var = samples.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n;
  (mean, var)
}


// Helper function to compute mean and variance
fn mean_strat_1(samples: &[f64]) -> f64 {
  let mut m = 0; // Adjusted total count
  samples.iter().map(|&x| {
    let adjusted_x = if x == 2.0 { 1.0 } else { 0.0 };
    if x == 2.0 || x == 0.0 { m += 1};
    adjusted_x
  }).sum::<f64>() / m as f64
}

// Helper function to compute mean and variance
fn mean_strat_2(samples: &[f64]) -> f64 {
  let mut m = 0; // Adjusted total count
  samples.iter().map(|&x| {
    let adjusted_x = if x == 1.0 { 1.0 } else { 0.0 };
    if x == 1.0 || x == 3.0 { m += 1};
    adjusted_x
  }).sum::<f64>() / m as f64
}

fn variance_given_mean(samples: &[f64], mean: f64) -> f64 {
  let n = samples.len() as f64;
  let total_samples = n / 0.75; // Total samples considering 3/4 are in the sample
  let exact_matches = total_samples / 4.0; // 1/4 of total samples are exact matches
  let m = total_samples + exact_matches; // Adjusted total count

  samples.iter().map(|&x| {
    let adjusted_x = if x == 2.0 { 1.0 } else { x };
    // if x == 2.0 { m += 1.0 }
    let weight = if x == 2.0 { 2.0 } else { 1.0 };
    weight * (adjusted_x - mean).powi(2)
  }).sum::<f64>() / m as f64
}

fn variance_given_mean_1(samples: &[f64], mean: f64) -> f64 {
  let n = samples.len() as f64;
  // let total_samples = n / 0.75; // Total samples considering 3/4 are in the sample
  // let exact_matches = total_samples / 4.0; // 1/4 of total samples are exact matches
  let mut m = 0; // Adjusted total count

  samples.iter().map(|&x| {
    let adjusted_x = if x == 2.0 { 1.0 } else { 0.0 };
    if x == 2.0 || x == 0.0 { m += 1 };
    // let weight = if x == 2.0 { 2.0 } else { 1.0 };
    if x == 2.0 || x == 0.0 {(adjusted_x - mean).powi(2) } else { 0.0 }
  }).sum::<f64>() / m as f64
}

fn variance_given_mean_2(samples: &[f64], mean: f64) -> f64 {
  let n = samples.len() as f64;
  // let total_samples = n / 0.75; // Total samples considering 3/4 are in the sample
  // let exact_matches = total_samples / 4.0; // 1/4 of total samples are exact matches
  let mut m = 0; // Adjusted total count

  samples.iter().map(|&x| {
    let adjusted_x = if x == 3.0 { 0.0 } else if x == 1.0 { 1.0 } else { 0.0 };
    if x == 1.0 || x == 3.0 { m += 1 };
    // let weight = if x == 2.0 { 2.0 } else { 1.0 };
    if x == 1.0 || x == 3.0 {(adjusted_x - mean).powi(2)} else { 0.0 }
  }).sum::<f64>() / m  as f64
}

pub fn run_task12() {
  let samples = 1_000_000;

  let plain_samples = monte_carlo_circle(samples);
  let plain2d_samples = monte_carlo_circle_2_d(samples);
  let strat_samples = stratified_sampling_circle(samples / 2);
  let antithetic_samples = antithetic_variates_circle(samples);

  let (plain_mean, plain_var) = mean_and_variance(&plain_samples);
  let (plain2d_mean, plain2d_var) = mean_and_variance(&plain2d_samples);


  // let strat_mean = mean_strat(&strat_samples);
  // let strat_var = variance_given_mean(&strat_samples, strat_mean);

  let strat_var1 = variance_given_mean_1(&strat_samples, mean_strat_1(&strat_samples));
  let strat_var2 = variance_given_mean_2(&strat_samples, mean_strat_2(&strat_samples));

  // let strat_mean = strat_samples.iter().sum::<f64>() / (strat_samples.len() as f64 * 2.0 ) as f64 + 0.25;
  let strat_mean = (mean_strat_1(&strat_samples) * 2.0 + mean_strat_2(&strat_samples) + 1.0) / 4.0 ;
  let strat_var = (strat_var1 * 2.0 + strat_var2) / 4.0;

  let (antithetic_mean, antithetic_var) = mean_and_variance(&antithetic_samples);

  println!("Plain Monte Carlo:     mean = {}, variance = {}", plain_mean, plain_var);
  println!("Plain Monte Carlo 2D:  mean = {}, variance = {}", plain2d_mean, plain2d_var);
  println!("Stratified Sampling:   mean = {}, variance = {}", strat_mean, strat_var);
  println!("Stratified Samplingdokladnie :   mean1 = {}, mean2 = {}", mean_strat_1(&strat_samples), mean_strat_1(&strat_samples));
  println!("Stratified Sampling:   strat_var1 = {}, strat_var2 = {}", strat_var1, strat_var2);
  println!("Antithetic Variates:   mean = {}, variance = {}", antithetic_mean, antithetic_var);
  println!("Exact value:                  {}", PI / 4.0);
}
// Sampling:   mean = 0.784913, variance = 0.09376403417197782
