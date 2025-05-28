use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

type Vertex = usize;
type Edge = (Vertex, Vertex);

#[derive(Clone)]
struct Graph {
  edges: Vec<Edge>,
  vertices: Vec<Vertex>,
}

impl Graph {
  fn new(edges: Vec<Edge>) -> Self {
    let mut vertices = vec![];
    for &(u, v) in &edges {
      if !vertices.contains(&u) {
        vertices.push(u);
      }
      if !vertices.contains(&v) {
        vertices.push(v);
      }
    }
    Graph { edges, vertices }
  }

  fn contract(&self) -> usize {
    let mut rng = thread_rng();
    let edges = self.edges.clone();
    let mut parent: HashMap<Vertex, Vertex> = HashMap::new();

    // Disjoint-set: init
    for &v in &self.vertices {
      parent.insert(v, v);
    }

    // Find representative of a set
    fn find(parent: &mut HashMap<Vertex, Vertex>, x: Vertex) -> Vertex {
      if parent[&x] != x {
        let p = parent[&x];
        let r = find(parent, p);
        parent.insert(x, r);
      }
      parent[&x]
    }

    // Union two sets
    fn union(parent: &mut HashMap<Vertex, Vertex>, x: Vertex, y: Vertex) {
      let x_root = find(parent, x);
      let y_root = find(parent, y);
      if x_root != y_root {
        parent.insert(x_root, y_root);
      }
    }

    let mut remaining_vertices = self.vertices.len();

    while remaining_vertices > 2 {
      let &(u, v) = edges.choose(&mut rng).unwrap();
      let u_rep = find(&mut parent, u);
      let v_rep = find(&mut parent, v);
      if u_rep != v_rep {
        union(&mut parent, u_rep, v_rep);
        remaining_vertices -= 1;
      }
    }

    // Count crossing edges
    edges
      .iter()
      .filter(|&&(u, v)| {
        find(&mut parent, u) != find(&mut parent, v)
      })
      .count()
  }

  fn repeated_karger(&self, iterations: usize) -> usize {
    let mut min_cut = usize::MAX;
    for _ in 0..iterations {
      let cut = self.contract();
      if cut < min_cut {
        min_cut = cut;
      }
    }
    min_cut
  }
}

fn main() {
  //  let edges = vec![
  //   (0, 1),
  //   (0, 2),
  //   (1, 2),
  //   (1, 3),
  //   (2, 3),
  // ];
  // Klika K 6
  let edges = vec![
    (0, 1),
    (0, 2),
    (0, 3),
    (0, 4),
    // (0, 5),
    (1, 2),
    (1, 3),
    (1, 4),
    (1, 5),
    (2, 3),
    (2, 4),
    (2, 5),
    (3, 4),
    (3, 5),
    (4, 5)
  ];
  let graph = Graph::new(edges);

  println!("Uruchamiam algorytm Kargera 100 razy...");
  let min_cut = graph.repeated_karger(100);
  println!("Przybliżone minimum cut: {}", min_cut);
}
