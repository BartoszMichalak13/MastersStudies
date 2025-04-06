use std::env;

fn horner(coeffs: &[i32], x: i32) -> i32 {
  let mut result = 0;
  for &coeff in coeffs.iter().rev() {
    result = result * x + coeff;
  }
  result
}

fn main() {
  let args: Vec<String> = env::args().skip(1).collect();

  let (x, coeffs): (i32, Vec<i32>) = if args.is_empty() {
    println!("Używam domyślnych danych: x = 3, coeffs = [2, -6, 2, -1]");
    (3, vec![2, -6, 2, -1])
  } else {
    let x = args[0].parse().unwrap_or_else(|_| {
      eprintln!("Nieprawidłowa wartość x, podaj liczbę całkowitą");
      std::process::exit(1);
    });

    let coeffs: Vec<i32> = args[1..].iter().map(|s| s.parse().unwrap_or(0)).collect();
    if coeffs.is_empty() {
      eprintln!("Podaj przynajmniej jeden współczynnik!");
      std::process::exit(1);
    }

    (x, coeffs)
  };

  let value = horner(&coeffs, x);
  println!("Wartość wielomianu dla x = {}: {}", x, value);
}
