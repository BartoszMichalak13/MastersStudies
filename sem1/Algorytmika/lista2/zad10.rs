use std::env;

//dopisac back tracking - wypisywanie najdluzszego wspolnego podciagu

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
  let mut i = la;
  let mut j = lb;
  let mut k = lc;
  let mut result = vec![];
  while i > 0 && j > 0 && k > 0 {
    if a[i - 1] == b[j - 1] && b[j - 1] == c[k - 1] {
      result.push(a[i - 1]);
      i -= 1;
      j -= 1;
      k -= 1;
    } else if dp[i][j][k] == dp[i - 1][j][k] {
      i -= 1;
    } else if dp[i][j][k] == dp[i][j - 1][k] {
      j -= 1;
    } else {
      k -= 1;
    }
  }
  println!("Najdłuższy wspólny podciąg: {}", result.iter().rev().collect::<String>());

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
