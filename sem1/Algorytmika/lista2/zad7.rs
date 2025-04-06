use std::env;
fn rabin_karp(text: &str, pattern: &str, prime: u64) -> Vec<usize> {
  let (n, m) = (text.len(), pattern.len());
  if m > n {
    return vec![];
  }

  let base = 256;
  let mut result = Vec::new();
  let (text_bytes, pattern_bytes) = (text.as_bytes(), pattern.as_bytes());

  let mut pattern_hash = 0;
  let mut text_hash = 0;
  let mut h = 1;

  for _ in 0..m - 1 {
    h = (h * base) % prime;
  }

  for i in 0..m {
    pattern_hash = (base * pattern_hash + pattern_bytes[i] as u64) % prime;
    text_hash = (base * text_hash + text_bytes[i] as u64) % prime;
  }

  for i in 0..=n - m {
    if pattern_hash == text_hash && &text[i..i + m] == pattern {
      result.push(i);
    }

    if i < n - m {
      text_hash = (base * (text_hash + prime - (text_bytes[i] as u64 * h) % prime)
        + text_bytes[i + m] as u64)
        % prime;
    }
  }

  result
}

fn main() {
  let args: Vec<String> = env::args().skip(1).collect();

  if args.len() < 2 {
    eprintln!("Użycie: ./zad7 <tekst> <wzorzec> <liczba_pierwsza>");
    return;
  }

  let text = &args[0];
  let pattern = &args[1];
  let prime = if args.len() > 2 {
    args[2].parse::<u64>().unwrap_or(101)
  } else {
    101
  };

  let positions = rabin_karp(text, pattern, prime);
  println!("Wzorzec znaleziony na pozycjach: {:?}", positions);
}
