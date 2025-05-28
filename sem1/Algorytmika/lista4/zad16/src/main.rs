use rand::Rng;
use std::collections::HashMap;

const M: usize = 1_000_000; // liczba kul
const N: usize = 1_000;     // liczba kubełków

fn random_allocation() -> Vec<usize> {
  let mut rng = rand::thread_rng();
  let mut bins = vec![0; N];

  for _ in 0..M {
    let bin = rng.gen_range(0..N);
    bins[bin] += 1;
  }

  bins
}

fn power_of_two_choices() -> Vec<usize> {
  let mut rng = rand::thread_rng();
  let mut bins = vec![0; N];

  for _ in 0..M {
    let i = rng.gen_range(0..N);
    let j = rng.gen_range(0..N);

    let target = if bins[i] <= bins[j] { i } else { j };
    bins[target] += 1;
  }

  bins
}

use std::fs::File;
use std::io::{Write, BufWriter};

fn analyze_with_csv(bins: &[usize], filename: &str) {
  let max_load = *bins.iter().max().unwrap();
  let empty_bins = bins.iter().filter(|&&x| x == 0).count();

  println!("Max load: {}", max_load);
  println!("Empty bins: {}", empty_bins);

  let mut histogram = HashMap::new();
  for &load in bins {
    *histogram.entry(load).or_insert(0) += 1;
  }

  let file = File::create(filename).expect("Unable to create file");
  let mut writer = BufWriter::new(file);

  writeln!(writer, "load,count").unwrap();
  let mut hist_sorted: Vec<_> = histogram.iter().collect();
  hist_sorted.sort_by_key(|&(load, _)| *load);
  for (load, count) in hist_sorted {
    writeln!(writer, "{},{}", load, count).unwrap();
  }

  println!("Histogram written to: {}", filename);
}

fn main() {
  println!("== Uniform Random Allocation ==");
  let bins1 = random_allocation();
  analyze_with_csv(&bins1, "uniform.csv");

  println!("\n== Power of Two Choices ==");
  let bins2 = power_of_two_choices();
  analyze_with_csv(&bins2, "power2.csv");
}

