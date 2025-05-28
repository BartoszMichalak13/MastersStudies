use rand::Rng;

fn approximate_count(values: &mut Vec<f64>) -> usize {
  values.sort_by(|a, b| a.partial_cmp(b).unwrap());
  let n = values.len();
  if n <= 400 {
    n
  } else {
    (399.0 / values[399]) as usize
  }
}

fn true_and_estimate(n: usize, trials: usize) -> (usize, f64) {
  let mut rel_errors = vec![];

  for _ in 0..trials {
    let mut samples: Vec<f64> = (0..n)
      .map(|_| rand::thread_rng().gen_range(0.0..1.0))
      .collect();
    let estimate = approximate_count(&mut samples);
    let rel_error = ((estimate as f64) - (n as f64)).abs() / (n as f64);
    rel_errors.push(rel_error);
  }

  let avg_rel_error = rel_errors.iter().sum::<f64>() / trials as f64;
  (n, avg_rel_error)
}

fn main() {
  println!("true_n,relative_error");

  let trials = 1000;
  for &n in &[100, 200, 400, 600, 800, 1000, 2000, 5000, 10000] {
    let (true_n, avg_rel_error) = true_and_estimate(n, trials);
    println!("{},{}", true_n, avg_rel_error);
  }
}
