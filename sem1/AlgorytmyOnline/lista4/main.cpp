#include <iostream>
#include <fstream>
#include <vector>
#include "graph/graph.hpp"
#include "algorithms/algorithm.hpp"
#include "distributions/distribution.hpp"

int main() {
  const int num_vertices = 64;
  const int num_requests = 65536;
  std::vector<int> D_values = {16, 32, 64, 128, 256};

  std::vector<std::shared_ptr<Graph>> graphs = {
    std::make_shared<Torus3D>(num_vertices),
    std::make_shared<Hypercube>(num_vertices)
  };

  std::vector<std::shared_ptr<Distribution>> distributions = {
    std::make_shared<UniformDistribution>(num_vertices),
    std::make_shared<HarmonicDistribution>(num_vertices),
    std::make_shared<DoubleHarmonicDistribution>(num_vertices)
  };

  std::ofstream csv("results.csv");
  csv << "Graph,Distribution,D,MoveToMin,CoinFlip\n";

  int M = 100;
  for (int i = 0; i < M; i++) {
    std::cout << "Iteration " << i + 1 << " of " << M << "\n";
    for (auto& graph : graphs) {
      for (auto& dist : distributions) {
        auto requests = dist->generate(num_requests);
        for (int D : D_values) {
          MoveToMin move_to_min_algo(graph, D);
          CoinFlip coin_flip_algo(graph, D);

          double cost1 = move_to_min_algo.simulate(requests);
          double cost2 = coin_flip_algo.simulate(requests);

          csv << graph->name() << "," << dist->name() << "," << D << ","
              << cost1 << "," << cost2 << "\n";
        }
      }
    }
  }
  csv.close();
  return 0;
}
