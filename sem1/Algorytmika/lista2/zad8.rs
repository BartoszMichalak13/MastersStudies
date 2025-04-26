use std::env;

fn lcs(x: &str, y: &str) -> usize {
  let (m, n) = (x.len(), y.len());
  let (xa, ya): (Vec<_>, Vec<_>) = (x.chars().collect(), y.chars().collect());
  let mut dp = vec![vec![0; n + 1]; m + 1];

  for i in 0..m {
    for j in 0..n {
      if xa[i] == ya[j] {
        dp[i + 1][j + 1] = dp[i][j] + 1;
      } else {
        dp[i + 1][j + 1] = dp[i + 1][j].max(dp[i][j + 1]);
      }
    }
  }

  dp[m][n]
}

fn generate_binary_strings(n: usize) -> Vec<String> {
  (0..1 << n)
    .map(|i| format!("{:0width$b}", i, width = n))
    .collect()
}

fn expected_lcs(n: usize) -> f64 {
  let strings = generate_binary_strings(n);
  let mut total = 0;
  let mut count = 0;

  for i in 0..strings.len() {
    for j in 0..strings.len() {
      total += lcs(&strings[i], &strings[j]);
      count += 1;
    }
  }

  total as f64 / count as f64
}

fn main() {
  let args: Vec<String> = env::args().skip(1).collect();

  if args.is_empty() {
    eprintln!("Użycie: ./zad8 <długość ciągu>");
    return;
  }

  let n: usize = args[0].parse().unwrap_or(5);
  let avg = expected_lcs(n);

  println!("Wartość oczekiwana E[LCS] dla długości {}: {:.4}", n, avg);
}
