use std::collections::{HashMap, HashSet};

type Vertex = usize;
type Edge = (Vertex, Vertex);

struct Graph {
  vertices: HashSet<Vertex>,
  edges: Vec<Edge>,
  adjacency: HashMap<Vertex, Vec<Vertex>>,
}

impl Graph {
  fn new(edges: Vec<Edge>) -> Self {
    let mut vertices = HashSet::new();
    let mut adjacency: HashMap<Vertex, Vec<Vertex>> = HashMap::new();
    for &(u, v) in &edges {
      vertices.insert(u);
      vertices.insert(v);
      adjacency.entry(u).or_default().push(v);
      adjacency.entry(v).or_default().push(u);
    }
    Graph { vertices, edges, adjacency }
  }

  fn derandomized_max_cut(&self) -> (HashSet<Vertex>, HashSet<Vertex>) {
    let mut a_k: HashSet<Vertex> = HashSet::new();
    let mut a_k_c: HashSet<Vertex> = HashSet::new();

    for &v in &self.vertices {
      let mut a = 0; // edges to A_k
      let mut b = 0; // edges to A_k^c
      if let Some(neighbors) = self.adjacency.get(&v) {
        for &u in neighbors {
          if a_k.contains(&u) {
            a += 1;
          } else if a_k_c.contains(&u) {
            b += 1;
          }
        }
      }

      if a <= b {
        a_k.insert(v);
      } else {
        a_k_c.insert(v);
      }
    }

    (a_k, a_k_c)
  }

  fn cut_size(&self, part1: &HashSet<Vertex>, part2: &HashSet<Vertex>) -> usize {
    self.edges
      .iter()
      .filter(|(u, v)| part1.contains(u) && part2.contains(v) || part1.contains(v) && part2.contains(u))
      .count()
  }
}

fn main() {
  // Przykładowy graf
  let edges = vec![
    (0, 1),
    (0, 2),
    (1, 2),
    (1, 3),
    (2, 3),
    (3, 4),
    (4, 5),
    (2, 5),
  ];

  let graph = Graph::new(edges);
  let (a_k, a_k_c) = graph.derandomized_max_cut();

  println!("A_k: {:?}", a_k);
  println!("A_k^c: {:?}", a_k_c);
  let cut_size = graph.cut_size(&a_k, &a_k_c);
  println!("Rozmiar cięcia: {}", cut_size);
}
