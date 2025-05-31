// main.cpp
#include <iostream>
#include <fstream>
#include <vector>
#include <random>
#include <string>
#include <iomanip>
#include "algorithms.hpp"
#include "distributions.hpp"




int main() {
  std::vector<std::string> algorithms = {"next_fit", "random_fit", "first_fit", "best_fit", "worst_fit"};
  // std::vector<std::string> distributions = {"uniform", "harmonic", "double_harmonic", "geometric"};

  std::vector<std::pair<std::string, std::vector<double>(*)(int)>> distributions = {
    {"uniform", uniform_dist},
    {"harmonic", harmonic_dist},
    {"double_harmonic", double_harmonic_dist},
    {"geometric", geometric_dist}
  };

  // std::mt19937 rng(42);
  std::random_device rd;
  std::mt19937 rng(rd());
  std::ofstream fout("output.csv");
  fout << "algorithm,distribution,run,bins_used,lower_bound,competitive_ratio\n";

  int runs = 100;
  int n = 10;
  int numberOfElements = 100;

  // for (const auto &dist : distributions) {
    for (auto& [dist_name, dist_fn] : distributions) {

    for (int r = 0; r < runs; ++r) {

      // auto base = generate_uniform_sequence(10, rng);
      auto items = generate_elemets(n, dist_fn(10), numberOfElements);
      // auto items = repeat_sequence(base, dist, rng);
      double sum = 0.0;
      for (double x : items) sum += x;
      int lower_bound = static_cast<int>(std::ceil(sum));

      for (const auto &alg : algorithms) {
        int bins = run_algorithm(items, alg, rng);
        double ratio = static_cast<double>(bins) / lower_bound;
        fout << alg << "," << dist_name << "," << r << "," << bins << "," << lower_bound << "," << std::fixed << std::setprecision(4) << ratio << "\n";
      }
    }
  }

  fout.close();
  std::cout << "Simulation complete. Results written to output.csv\n";
  return 0;
}



// int main() {
//   const int request_len = 100000;
//   std::ofstream out("results.csv");
//   out << "n,k,distribution,algorithm,cost\n";

//   std::vector<int> ns = {20, 30, 40, 50, 60, 70, 80, 90, 100};
//   std::vector<std::pair<std::string, std::vector<double>(*)(int)>> distributions = {
//       {"uniform", uniform_dist},
//       {"harmonic", harmonic_dist},
//       {"double_harmonic", double_harmonic_dist},
//       {"geometric", geometric_dist}
//   };

//   for (int i = 0; i < 100; ++i) {

//     for (int n : ns) {
//         std::vector<int> ks;
//         for (int k = n / 10; k <= n / 5; ++k) ks.push_back(k);

//         for (auto& [dist_name, dist_fn] : distributions) {
//             auto probs = dist_fn(n);
//             auto requests = generate_requests(n, probs, request_len);

//             // for (int i = 0; i < 100; ++i) {

//               for (int k : ks) {
//                   std::vector<std::pair<std::string, double>> results = {
//                       {"FIFO", simulate_fifo(requests, k)},
//                       {"FWF", simulate_fwf(requests, k)},
//                       {"LRU", simulate_lru(requests, k)},
//                       {"LFU", simulate_lfu(requests, k)},
//                       {"RAND", simulate_rand(requests, k)},
//                       {"RMA", simulate_rma(requests, k)}
//                   };

//                   for (auto& [alg, cost] : results) {
//                       out << n << "," << k << "," << dist_name << "," << alg << "," << cost << "\n";
//                   }

//                   std::cout << "Done: n=" << n << " k=" << k << " dist=" << dist_name << "\n";
//               }
//             // }
//         }
//     }
//   }

//   out.close();
//   std::cout << "All results written to results.csv\n";
//   return 0;
// }
