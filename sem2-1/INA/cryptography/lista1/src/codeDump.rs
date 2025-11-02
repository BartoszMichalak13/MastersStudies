use md5::{Md5, Digest};
use std::fs;
use std::env;
use std::fmt::Write;

/// Compute MD5(data)
fn md5_of(data: &[u8]) -> [u8; 16] {
    let mut hasher = Md5::new();
    hasher.update(data);
    let result = hasher.finalize();
    let mut out = [0u8; 16];
    out.copy_from_slice(&result);
    out
}

/// Compute MD5(MD5(IV, M0), M1)
fn nested_md5(m0: &[u8], m1: &[u8]) -> [u8; 16] {
    let inner = md5_of(m0);
    let mut hasher = Md5::new();
    hasher.update(&inner);
    hasher.update(m1);
    let result = hasher.finalize();
    let mut out = [0u8; 16];
    out.copy_from_slice(&result);
    out
}

/// Convert bytes to lowercase hex
fn to_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}

// /// Read a hex file and convert to raw bytes using `hex` crate
// fn read_hex_file(path: &str) -> Vec<u8> {
//     let text = fs::read_to_string(path)
//         .unwrap_or_else(|_| panic!("Cannot read file {}", path));
//     // remove all whitespace
//     let clean: String = text.chars().filter(|c| !c.is_whitespace()).collect();
//     hex::decode(clean).expect("Invalid hex in file")
// }

fn read_hex_file(path: &str) -> Vec<u8> {
    let text = std::fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Cannot read file {}", path));

    // Split on any whitespace, pad each hex word to 8 chars if needed,
    // then concatenate all together into one long hex string.
    let mut normalized = String::new();
    for word in text.split_whitespace() {
        let mut w = word.trim().to_string();
        // pad on the left with zeros until it's 8 characters long
        while w.len() < 8 {
            w.insert(0, '0');
        }
        normalized.push_str(&w);
    }

    println!("Normalized hex: {}", normalized);
    // decode to bytes
    hex::decode(normalized).expect("Invalid hex in file")
}

fn fix_endianness(block: &mut [u8]) {
    assert_eq!(block.len() % 4, 0);
    for chunk in block.chunks_exact_mut(4) {
        chunk.reverse();
    }
}

/// Verify equality of H1 and H2
fn verify_collision(m0: &[u8], m0p: &[u8], m1: &[u8], m1p: &[u8]) {
    let h1 = nested_md5(m0, m1);
    let h2 = nested_md5(m0p, m1p);

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

    println!("Reading messages from files:");
    println!(" M0: {}", args[1]);
    let mut m0 = read_hex_file(&args[1]);
    fix_endianness(&mut m0);
    println!(" M0p': {}", args[2]);
    let mut m0p = read_hex_file(&args[2]);
    fix_endianness(&mut m0p);
    println!(" M1: {}", args[3]);
    let mut m1 = read_hex_file(&args[3]);
    fix_endianness(&mut m1);
    println!(" M1p': {}", args[4]);
    let mut m1p = read_hex_file(&args[4]);
    fix_endianness(&mut m1p);

    verify_collision(&m0, &m0p, &m1, &m1p);
}

///md-5 = "0.10.6"
