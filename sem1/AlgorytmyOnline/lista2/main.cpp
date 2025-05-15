#include <iostream>
#include <fstream>
#include <vector>
#include <string>
#include "distributions.hpp"
#include "cache_algorithms.hpp"

int main() {
    const int request_len = 100000;
    std::ofstream out("results.csv");
    out << "n,k,distribution,algorithm,cost\n";

    std::vector<int> ns = {20, 30, 40, 50, 60, 70, 80, 90, 100};
    std::vector<std::pair<std::string, std::vector<double>(*)(int)>> distributions = {
        {"uniform", uniform_dist},
        {"harmonic", harmonic_dist},
        {"double_harmonic", double_harmonic_dist},
        {"geometric", geometric_dist}
    };

    for (int i = 0; i < 100; ++i) {

      for (int n : ns) {
          std::vector<int> ks;
          for (int k = n / 10; k <= n / 5; ++k) ks.push_back(k);

          for (auto& [dist_name, dist_fn] : distributions) {
              auto probs = dist_fn(n);
              auto requests = generate_requests(n, probs, request_len);

              // for (int i = 0; i < 100; ++i) {

                for (int k : ks) {
                    std::vector<std::pair<std::string, double>> results = {
                        {"FIFO", simulate_fifo(requests, k)},
                        {"FWF", simulate_fwf(requests, k)},
                        {"LRU", simulate_lru(requests, k)},
                        {"LFU", simulate_lfu(requests, k)},
                        {"RAND", simulate_rand(requests, k)},
                        {"RMA", simulate_rma(requests, k)}
                    };

                    for (auto& [alg, cost] : results) {
                        out << n << "," << k << "," << dist_name << "," << alg << "," << cost << "\n";
                    }

                    std::cout << "Done: n=" << n << " k=" << k << " dist=" << dist_name << "\n";
                }
              // }
          }
      }
    }

    out.close();
    std::cout << "All results written to results.csv\n";
    return 0;
}
