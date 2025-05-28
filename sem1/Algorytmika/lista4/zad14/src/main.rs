fn derandomized_generate_zero_string(n: usize) -> Vec<u8> {
  let mut result = Vec::with_capacity(n);
  for _ in 0..n {
    // zawsze wybieramy 0, bo tylko wtedy sukces jest możliwy
    result.push(0);
  }
  result
}

fn main() {
  let n = 10;
  let zero_string = derandomized_generate_zero_string(n);
  println!("Wynik derandomizacji: {:?}", zero_string);
}
