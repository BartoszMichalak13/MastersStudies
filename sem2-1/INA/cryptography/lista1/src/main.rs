use md5::{Digest, Md5};
use std::env;
use std::fmt::Write;
use std::fs;

// const M0_HEX : &str = "02dd31d1c4eee6c5069a3d695cf9af9887b5ca2fab7e46123e580440897ffbb80634ad5502b3f4098388e4835a417125e82551089fc9cdf7f2bd1dd95b3c3780";
// const M1_HEX : &str = "d11d0b969c7b41dcf497d8e4d555655ac79a73350cfdebf066f129308fb109d1797f2775eb5cd530baade8225c15cc79ddcb74ed6dd3c55fd80a9bb1e3a7cc35";
// const M0_PRIME_HEX : &str = "02dd31d1c4eee6c5069a3d695cf9af9807b5ca2fab7e46123e580440897ffbb80634ad5502b3f4098388e4835a41f125e82551089fc9cdf772bd1dd95b3c3780";
// const M1_PRIME_HEX : &str = "d11d0b969c7b41dcf497d8e4d555655a479a73350cfdebf066f129308fb109d1797f2775eb5cd530baade8225c154c79ddcb74ed6dd3c55f580a9bb1e3a7cc35";

// const H_HEX: &str = "9603161fa30f9dbf9f65ffbcf41fc7ef";
// const H_STAR_HEX: &str = "a4c0d35c95a63a805915367dcfe6b751";

fn read_hex_file(path: &str) -> [u8; 64] {
  let text = fs::read_to_string(path).expect("Cannot read file");
  let mut normalized = String::new();
  for word in text.split_whitespace() {
    let mut w = word.trim().to_string();
    while w.len() < 8 { w.insert(0, '0'); }
    normalized.push_str(&w);
  }
  let mut bytes = hex::decode(normalized).expect("Invalid hex");
  assert_eq!(bytes.len(), 64, "Each block must be 64 bytes");
  // for chunk in bytes.chunks_exact_mut(4) { chunk.reverse(); } // fix endian
  bytes.try_into().unwrap()
}

/// Convert bytes to lowercase hex
fn to_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}

/// Verify equality of H1 and H2
fn verify_collision(m0: &[u8], m0p: &[u8], m1: &[u8], m1p: &[u8]) {

    let mut hasher1 = Md5::new();
    hasher1.update(&m0);
    hasher1.update(&m1);
    let h1 = hasher1.finalize();

    let mut hasher2 = Md5::new();
    hasher2.update(&m0p);
    hasher2.update(&m1p);
    let h2 = hasher2.finalize();

    println!("H1 = {}", to_hex(&h1));
    println!("H2 = {}", to_hex(&h2));

    if h1 == h2 {
        println!("✅ Collision verified: H1 == H2");
    } else {
        println!("❌ No collision: H1 != H2");
    }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() != 5 {
    eprintln!("Usage: {} <M0.hex> <M0p.hex> <M1.hex> <M1p.hex>", args[0]);
    std::process::exit(1);
  }

  let m0 = read_hex_file(&args[1]);
  let m0p = read_hex_file(&args[2]);
  let m1 = read_hex_file(&args[3]);
  let m1p = read_hex_file(&args[4]);
  verify_collision(&m0, &m0p, &m1, &m1p);

}