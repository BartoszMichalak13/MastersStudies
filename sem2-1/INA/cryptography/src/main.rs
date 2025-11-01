use md5::Context;
use std::fs::File;
use std::io::{self, Read};

// fn main() {
//     println!("Hello, world!");
//     let digest = md5::compute(b"abcdefghijklmnopqrstuvwxyz");
//     assert_eq!(format!("{:x}", digest), "c3fcd3d76192e4007dfb496cca67e13b");
//     println!("MD5 digest: {:x}\n", digest);
// }

fn main() -> io::Result<()> {
    let mut context = Context::new();
    // let mut file = File::open("large_file.bin")?;
    let mut file = File::open("md5.c")?;
    let mut buffer = [0u8; 4096];

    while let Ok(n) = file.read(&mut buffer) {
        if n == 0 {
            break;
        }
        context.consume(&buffer[..n]);
    }

    let digest = context.finalize();
    // println!("MD5 = {:x}", digest);
    println!("MD5 = {digest:x}");
    Ok(())
}
