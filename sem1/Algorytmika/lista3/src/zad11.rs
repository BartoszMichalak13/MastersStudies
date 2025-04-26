// zad11.rs
use rand::prelude::*;
use std::f64::consts::PI;

pub fn run_task11() {
  let samples = 1_000_000;
  let mut rng = thread_rng();
  let mut total = 0.0;

  for _ in 0..samples {
    let x = rng.gen_range(0.0..PI);
    total += x.sin();
  }

  let estimate = (PI * total) / samples as f64;
  println!("Monte Carlo approximation of ∫₀^π sin(x) dx: {}", estimate);
  println!("Exact value: 2.0");
}
