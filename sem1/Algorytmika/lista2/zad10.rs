use std::env;

fn lcs3(a: &str, b: &str, c: &str) -> usize {
  let (la, lb, lc) = (a.len(), b.len(), c.len());
  let (a, b, c): (Vec<_>, Vec<_>, Vec<_>) =
    (a.chars().collect(), b.chars().collect(), c.chars().collect());

  let mut dp = vec![vec![vec![0; lc + 1]; lb + 1]; la + 1];

  for i in 0..la {
    for j in 0..lb {
      for k in 0..lc {
        if a[i] == b[j] && b[j] == c[k] {
          dp[i + 1][j + 1][k + 1] = dp[i][j][k] + 1;
        } else {
          dp[i + 1][j + 1][k + 1] = *[
            dp[i][j + 1][k + 1],
            dp[i + 1][j][k + 1],
            dp[i + 1][j + 1][k],
          ]
          .iter()
          .max()
          .unwrap();
        }
      }
    }
  }

  dp[la][lb][lc]
}

fn main() {
  let args: Vec<String> = env::args().skip(1).collect();

  if args.len() < 3 {
    eprintln!("Użycie: ./zad10 <ciag1> <ciag2> <ciag3>");
    return;
  }

  let (a, b, c) = (&args[0], &args[1], &args[2]);
  let result = lcs3(a, b, c);

  println!("Długość LCS trzech ciągów: {}", result);
}
