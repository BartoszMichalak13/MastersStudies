use std::env;
use std::fmt::Write;
use std::fs;

/// MD5 auxiliary functions and constants
fn f(x: u32, y: u32, z: u32) -> u32 { (x & y) | (!x & z) }
fn g(x: u32, y: u32, z: u32) -> u32 { (x & z) | (y & !z) }
fn h(x: u32, y: u32, z: u32) -> u32 { x ^ y ^ z }
fn ii(x: u32, y: u32, z: u32) -> u32 { y ^ (x | !z) } // renamed from `i`
fn rotl(x: u32, n: u32) -> u32 { x.rotate_left(n) }

const S: [[u32; 4]; 4] = [
  [7, 12, 17, 22],
  [5, 9, 14, 20],
  [4, 11, 16, 23],
  [6, 10, 15, 21],
];

const K: [u32; 64] = [
  0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
  0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
  0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
  0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
  0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
  0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
  0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
  0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];

/// One MD5 compression step: returns new [A,B,C,D]
fn md5_compress(iv: [u32; 4], block: &[u8; 64]) -> [u32; 4] {
  let mut a = iv[0];
  let mut b = iv[1];
  let mut c = iv[2];
  let mut d = iv[3];

  let mut m = [0u32; 16];
  for (i, chunk) in block.chunks_exact(4).enumerate() {
    m[i] = u32::from_le_bytes(chunk.try_into().unwrap());
  }

  for i in 0..64 {
    let (f_val, g) = match i {
      0..=15 => (f(b, c, d), i),
      16..=31 => (g(b, c, d), (5 * i + 1) % 16),
      32..=47 => (h(b, c, d), (3 * i + 5) % 16),
      _ => (ii(b, c, d), (7 * i) % 16), // <-- renamed function
    };

    let temp = d;
    d = c;
    c = b;
    b = b.wrapping_add(rotl(a.wrapping_add(f_val).wrapping_add(m[g]).wrapping_add(K[i]), S[i / 16][i % 4]));
    a = temp;
  }

  [
    iv[0].wrapping_add(a),
    iv[1].wrapping_add(b),
    iv[2].wrapping_add(c),
    iv[3].wrapping_add(d),
  ]
}

/// Convert u32 state to hex string
fn to_hex_state(state: [u32; 4]) -> String {
  state.iter().flat_map(|v| v.to_le_bytes()).map(|b| format!("{:02x}", b)).collect()
}

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
  for chunk in bytes.chunks_exact_mut(4) { chunk.reverse(); } // fix endian
  bytes.try_into().unwrap()
}

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() != 5 {
    eprintln!("Usage: {} <M0.hex> <M0p.hex> <M1.hex> <M1p.hex>", args[0]);
    std::process::exit(1);
  }

  let mut m0 = read_hex_file(&args[1]);
  let mut m0p = read_hex_file(&args[2]);
  let mut m1 = read_hex_file(&args[3]);
  let mut m1p = read_hex_file(&args[4]);

  let iv = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476];

  let h1 = md5_compress(iv, &m0);
  let h1p = md5_compress(iv, &m0p);
  let h2 = md5_compress(h1, &m1);
  let h2p = md5_compress(h1p, &m1p);

  println!("H  = {}", to_hex_state(h2));
  println!("H' = {}", to_hex_state(h2p));
  println!("{}", if h2 == h2p { "✅ Collision verified" } else { "❌ No collision" });
}
