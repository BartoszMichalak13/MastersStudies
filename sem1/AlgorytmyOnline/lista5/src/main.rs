use rand::distributions::{Bernoulli};
use rand::{Rng, SeedableRng};
use rand::rngs::{StdRng, OsRng};
use rand::RngCore;
use rand::distributions::Distribution;

const NODES: usize = 64;
const REQUESTS: usize = 65536;
const REPEATS: usize = 50;

#[derive(Clone, Copy)]
enum RequestType {
  Read,
  Write,
}

// struct Simulation {
//   has_copy: [bool; NODES],
//   copy_count: usize,
//   d: usize,
//   rng: StdRng,
// }

// impl Simulation {
//   fn new(d: usize, seed: u64) -> Self {
//     let mut has_copy = [false; NODES];
//     has_copy[0] = true;
//     Self {
//       has_copy,
//       copy_count: 1,
//       d,
//       rng: StdRng::seed_from_u64(seed),
//     }
//   }

//   fn generate_request(&mut self, p: f64) -> (usize, RequestType) {
//     let node = self.rng.gen_range(0..NODES);
//     let is_write = Bernoulli::new(p).unwrap().sample(&mut self.rng);
//     let request = if is_write { RequestType::Write } else { RequestType::Read };
//     (node, request)
//   }

//   fn handle_request(&mut self, node: usize, request: RequestType) -> usize {
//     match request {
//       RequestType::Read => {
//         if self.has_copy[node] {
//           0
//         } else {
//           1
//         }
//       }
//       RequestType::Write => {
//         if self.has_copy[node] {
//           self.copy_count - 1
//         } else {
//           self.has_copy[node] = true;
//           self.copy_count += 1;
//           self.d + self.copy_count
//         }
//       }
//     }
//   }
// }

struct Simulation {
  has_copy: [bool; NODES],
  counters: [usize; NODES],
  copy_count: usize,
  d: usize,
  rng: StdRng,
}

impl Simulation {
  fn new(d: usize, seed: u64) -> Self {
    let mut has_copy = [false; NODES];
    has_copy[0] = true;
    Self {
      has_copy,
      counters: [0; NODES],
      copy_count: 1,
      d,
      rng: StdRng::seed_from_u64(seed),
    }
  }

  fn generate_request(&mut self, p: f64) -> (usize, RequestType) {
    let node = self.rng.gen_range(0..NODES);
    let is_write = Bernoulli::new(p).unwrap().sample(&mut self.rng);
    let request = if is_write { RequestType::Write } else { RequestType::Read };
    (node, request)
  }

  fn is_waiting(&self) -> Option<usize> {
    if self.copy_count == 1 {
      self.has_copy.iter().position(|&x| x)
    } else {
      None
    }
  }

  fn handle_request(&mut self, node: usize, request: RequestType) -> usize {
    let waiting_node = self.is_waiting();
    let is_holder = self.has_copy[node];
    let mut cost = 0;

    // step 1 read i write => cost += 1
    // step 2 if c_p == d => replicate, cost += d
    // step 3 write => decrement counters of other nodes
    // step 4 if c == 0 and copyCount == 1 => wait
    // step 5 if c == 0 and copyCount > 1 => delete own copy, copyCount -= 1

    match request {
      RequestType::Read => {
        if is_holder {
          // Holder reads for free
          cost = 0;
        } else {
          // Step 1: increment own counter
          self.counters[node] += 1;
          cost = 1;

          // Step 2: replicate if needed
          if self.counters[node] == self.d {
            self.has_copy[node] = true;
            self.copy_count += 1;
            cost += self.d;
            // println!("Node {} replicated, copy count: {}", node, self.copy_count);
          }
        }
      }
      RequestType::Write => {
        if is_holder {
          // Writing to own copy
          cost = self.copy_count - 1;
          // cost = 0;


          // // Step 4: if it's the last copy, wait
          // if self.copy_count == 1 {
          //   // Wait until someone else replicates
          //   // Simulation waits: nothing happens, no deletion yet
          //   // Deletion happens after someone replicates
          // } else

           if self.counters[node] == 0 && self.copy_count > 1 {
            // Step 5: delete own copy
            self.has_copy[node] = false;
            self.copy_count -= 1;
            self.counters[node] = 0;
          }
        } else {
          // Step 1: increment own counter if waiting
          // if let Some(_) = waiting_node {
            self.counters[node] += 1;
          // }

          // Step 2: replicate if needed
          if self.counters[node] == self.d {
            self.has_copy[node] = true;
            self.copy_count += 1;
            cost = self.d + self.copy_count;
          } else {
            cost = self.copy_count;
          }
        }

        // Step 3: decrement other counters
        for i in 0..NODES {
          if i != node && self.counters[i] > 0 && self.has_copy[i] {
            self.counters[i] -= 1;
          }
        }
      }
    }
    cost
  }
}

// fn run_single_simulation(d: usize, p: f64, seed: u64, repeat: usize) -> (f64, usize) {
//   let mut sim = Simulation::new(d, seed);
//   let mut total_cost = 0;
//   let mut max_copies = 1;

//   for i in 0..REQUESTS {
//     let mut cost = 0;
//     let (node, req_type) = sim.generate_request(p);
//     cost = sim.handle_request(node, req_type);
//     total_cost += cost;
//     if sim.copy_count > max_copies {
//       max_copies = sim.copy_count;
//     }
//     // println!("D: {}, P: {}, Cost: {}, sim.copy_count: {}, Request: {}", d, p, cost, sim.copy_count, i,);
//     println!("{}, {}, {}, {}, {}, {}", d, p, cost, sim.copy_count, i, repeat);
//   }

//   let avg_cost = total_cost as f64 / REQUESTS as f64;
//   (avg_cost, max_copies)
// }

// fn run_simulation_repeated(d: usize, p: f64) -> (f64, f64) {
//   let mut total_cost = 0.0;
//   let mut total_max = 0.0;

//   let mut os_rng = OsRng;

//   for i in 0..REPEATS {
//     let seed = os_rng.next_u64();
//     let (cost, max_copies) = run_single_simulation(d, p, seed, i);
//     total_cost += cost;
//     total_max += max_copies as f64;
//     // println!("Seed: {}, Cost: {:.2}, Max Copies: {}", seed, cost, max_copies);
//     // println!("Repeat: {}, D: {}, P: {} Cost: {:.2}, Max Copies: {}", _, d, p, cost, max_copies);
//   }

//   (total_cost / REPEATS as f64, total_max / REPEATS as f64)
// }

// fn main() {
//   let ds = [16, 32, 64, 128, 256];
//   // let ds = (1..512).collect::<Vec<_>>();
//   // let ds = (1..64).collect::<Vec<_>>();
//   let ps = [0.01, 0.02, 0.05, 0.1, 0.2, 0.5];
//   // let ps = (1..=50).map(|x| x as f64 / 100.0).collect::<Vec<_>>();

//   // println!("D,p,AverageCost,AvgMaxCopies");
//   println!("D,p,Cost,Copies,Requests,Repeats");

//   for &d in &ds {
//     for &p in &ps {
//       let (avg_cost, avg_max) = run_simulation_repeated(d, p);
//       // println!("{},{},{:.4},{:.2}", d, p, avg_cost, avg_max);
//     }
//   }
// }
fn run_single_simulation(
  d: usize,
  p: f64,
  seed: u64,
) -> (Vec<usize>, Vec<usize>) {
  let mut sim = Simulation::new(d, seed);
  let mut costs = vec![0; REQUESTS];
  let mut copies = vec![0; REQUESTS];

  for i in 0..REQUESTS {
    let (node, req_type) = sim.generate_request(p);
    let cost = sim.handle_request(node, req_type);
    costs[i] = cost;
    copies[i] = sim.copy_count;
    // println!("{}, {}, {}, {}, {}, {}", d, p, cost, sim.copy_count, i, req_type as usize);
  }

  (costs, copies)
}

fn run_simulation_repeated(d: usize, p: f64) {
  let mut total_costs = vec![0.0; REQUESTS];
  let mut total_copies = vec![0.0; REQUESTS];
  let mut os_rng = OsRng;

  for j in 0..REPEATS {
    let seed = os_rng.next_u64();
    let (costs, copies) = run_single_simulation(d, p, seed);
    for i in 0..REQUESTS {
      total_costs[i] += costs[i] as f64;
      total_copies[i] += copies[i] as f64;
    }
    if j == 0 {
      let mut max_copies = 0.0;
      for k in 0..REQUESTS {
        if total_copies[k] > max_copies {
          max_copies = total_copies[k];
        }
      }
      // let max_copies = total_copies.iter().max().unwrap();
      println!("max_copies: {}, d: {}, p: {}", max_copies, d, p);
    }
  }

  for i in 0..REQUESTS {
    let avg_cost = total_costs[i] / REPEATS as f64;
    let avg_copies = total_copies[i] / REPEATS as f64;
    // println!("{},{},{:.4},{:.2},{}", d, p, avg_cost, avg_copies, i);
  }
}

fn main() {
  let ds = [16, 32, 64, 128, 256];
  let ps = [0.01, 0.02, 0.05, 0.1, 0.2, 0.5];

  println!("D,p,AverageCost,AvgMaxCopies,Request");

  for &d in &ds {
    for &p in &ps {
      run_simulation_repeated(d, p);
    }
  }
}
