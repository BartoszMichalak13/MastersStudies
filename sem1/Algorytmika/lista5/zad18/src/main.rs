use rand::Rng;
use std::time::Instant;

fn generate_change_moments_online(n: usize) -> Vec<usize> {
  let mut rng = rand::thread_rng();
  let mut current = 1; // pierwszy element zawsze w próbce
  let mut moments = Vec::new();
  let mut count = 0;

  for i in 2..=5_000_000_000 {
    let prob = 1.0 / i as f64;
    if rng.gen_bool(prob) {
      moments.push(i);
      current = i;
      if count <= n/2 {
        count += 1;
        // println!("Moment {}: {}", count, i);
      } else {
        break;
      }
    }
  }

  moments
}

fn generate_change_moments_offline(n: usize) -> Vec<usize> {
  let mut rng = rand::thread_rng();
  let mut i = 1;
  let mut moments = Vec::new();

  while moments.len() < n {
    let u: f64 = rng.gen(); // losowe z [0,1)
    let skip = (u * i as f64 / (1.0 - u)).ceil() as usize;
    i += skip;
    moments.push(i);
  }

  moments
}

fn main() {
  let n = 50;

  println!("Measure time for n = {}", n);

  let start_online = Instant::now();
  let online = generate_change_moments_online(n);
  let duration_online = start_online.elapsed();
  println!("Online change moments (up to 50):");
  println!("{:?}", &online[..50.min(online.len())]);
  // println!("Total online moments: {}", online.len());
  println!("Online execution time: {:?}", duration_online);

  let start_offline = Instant::now();
  let offline = generate_change_moments_offline(n);
  let duration_offline = start_offline.elapsed();
  println!("\nOffline change moments (up to 50):");
  println!("{:?}", &offline[..50.min(offline.len())]);
  println!("Offline execution time: {:?}", duration_offline);
  // println!("Online change moments (up to 50):");
  // let online = generate_change_moments_online(n);
  // println!("{:?}", &online[..50.min(online.len())]);
  // println!("Total online moments: {}", online.len());

  // println!("\nOffline change moments (up to 50):");
  // let offline = generate_change_moments_offline(n);
  // println!("{:?}", &offline[..50.min(offline.len())]);
}
