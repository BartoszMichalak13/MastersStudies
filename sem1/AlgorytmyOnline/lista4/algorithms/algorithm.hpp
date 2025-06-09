#pragma once
#include <memory>
#include <vector>
#include "../graph/graph.hpp"
#include "../utils/rng.hpp"

class Algorithm {
public:
  virtual ~Algorithm() = default;
  Algorithm(std::shared_ptr<Graph> graph, int D) : graph(graph), D(D) {};
  virtual double simulate(const std::vector<int>& requests) = 0;

protected:
  std::shared_ptr<Graph> graph;
  int D;
};

class MoveToMin : public Algorithm {
public:
  MoveToMin(std::shared_ptr<Graph> g, int D) : Algorithm(g, D) {this->graph = g; this->D = D;}

  double simulate(const std::vector<int>& requests) override {
    int server = requests[0];
    double cost = 0.0;
    for (size_t i = 0; i < requests.size(); i += D) {
      // Wyznacz koniec aktualnej fazy
      size_t phase_end = std::min(i + D, requests.size());

      // Obsłuż żądania w fazie — bez przemieszczania zasobu
      for (size_t j = i; j < phase_end; ++j) {
        cost += graph->distance(server, requests[j]);
      }

      // Na koniec fazy przenosimy zasób do najlepszego wierzchołka
      // tylko jeśli jeszcze są żądania (czyli nie jesteśmy na końcu)
      if (phase_end == requests.size()) break;

      int best_node = server;
      int min_total = std::numeric_limits<int>::max();

      for (int v = 0; v < 64; ++v) {
        int total_dist = 0;
        for (size_t j = i; j < phase_end; ++j) {
          total_dist += graph->distance(v, requests[j]);
        }
        if (total_dist < min_total) {
          min_total = total_dist;
          best_node = v;
        }
      }

      cost += D * graph->distance(server, best_node);  // koszt przemieszczenia
      server = best_node;
    }

    return cost;
  }

private:
  std::shared_ptr<Graph> graph;
  int D;
};

class CoinFlip : public Algorithm {
public:
  CoinFlip(std::shared_ptr<Graph> g, int D) : Algorithm(g, D) {this->graph = g; this->D = D;}

  double simulate(const std::vector<int>& requests) override {
    int server = requests[0];
    double cost = 0.0;
    std::mt19937& gen = rng();
    std::uniform_real_distribution<double> prob(0.0, 1.0);

    for (int r : requests) {
      int dist = graph->distance(server, r);
      cost += dist;

      // Z prawdopodobieństwem 1 / (2D) wykonujemy migrację
      if (prob(gen) < 1.0 / (2.0 * D)) {
        cost += D * graph->distance(server, r);
        server = r;
      }
    }

    return cost;
  }
};
