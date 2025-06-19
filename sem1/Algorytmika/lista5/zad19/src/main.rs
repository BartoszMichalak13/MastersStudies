use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Write;

struct HyperLogLog {
  m: usize,
  b: u8,
  registers: Vec<u8>,
  alpha: f64,
}

impl HyperLogLog {
  fn new(b: u8) -> Self {
    let m = 1 << b;
    let alpha = match m {
      16 => 0.673,
      32 => 0.697,
      64 => 0.709,
      _ if m >= 128 => 0.7213 / (1.0 + 1.079 / m as f64),
      _ => panic!("Unsupported m = {}", m),
    };
    Self {
      m,
      b,
      registers: vec![0; m],
      alpha,
    }
  }

  fn add<T: Hash>(&mut self, item: &T) {
    let mut hasher = DefaultHasher::new();
    item.hash(&mut hasher);
    let hash = hasher.finish();
    let j = (hash >> (64 - self.b)) as usize;
    let w = hash << self.b;
    let rank = w.leading_zeros() + 1;
    self.registers[j] = self.registers[j].max(rank as u8);
  }

  fn estimate(&self) -> f64 {
    let z: f64 = self
      .registers
      .iter()
      .map(|&r| 2f64.powi(-(r as i32)))
      .sum();
    let raw_estimate = self.alpha * (self.m as f64).powi(2) / z;

    let two_to_32 = 2f64.powi(32);
    let threshold = (1.0 / 30.0) * two_to_32;

    let v = self.registers.iter().filter(|&&r| r == 0).count();

    let estimate = if raw_estimate <= 2.5 * (self.m as f64) {
      if v != 0 {
        (self.m as f64) * ((self.m as f64) / v as f64).ln()
      } else {
        raw_estimate
      }
    } else if raw_estimate <= threshold {
      raw_estimate
    } else {
      -two_to_32 * (1.0 - raw_estimate / two_to_32).ln()
    };

    estimate
  }
}

fn main() {
  let mut file = File::create("results.csv").expect("Unable to create file");
  writeln!(file, "n,n_hat,n_hat_over_n,m").expect("Write failed");

  let mut global_counter = 1u64;

  for b in 16..=32 {
    let m = 1 << b;
    for n in 1..=10_000 {
      let mut hll = HyperLogLog::new(b);
      for _ in 0..n {
        hll.add(&global_counter);
        global_counter += 1;
      }
      let n_hat = hll.estimate();
      let ratio = n_hat / n as f64;
      writeln!(file, "{},{},{},{}", n, n_hat, ratio, m).expect("Write failed");
    }
  }
}
