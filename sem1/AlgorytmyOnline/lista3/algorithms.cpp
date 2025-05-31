#include "algorithms.hpp"
#include <algorithm>

int run_algorithm(const std::vector<double> &items, const std::string &name, std::mt19937 &rng) {
  std::vector<double> bins;

  if (name == "next_fit") {
    double current = 0;
    for (double item : items) {
      if (current + item > 1.0) {
        bins.push_back(current);
        current = item;
      } else {
        current += item;
      }
    }
    if (current > 0) bins.push_back(current);
  }

  else if (name == "random_fit") {
    for (double item : items) {
      std::vector<int> candidates;
      for (int i = 0; i < bins.size(); ++i) {
        if (bins[i] + item <= 1.0) candidates.push_back(i);
      }
      if (!candidates.empty()) {
        std::uniform_int_distribution<int> dist(0, candidates.size() - 1);
        bins[candidates[dist(rng)]] += item;
      } else {
        bins.push_back(item);
      }
    }
  }

  else if (name == "first_fit") {
    for (double item : items) {
      bool placed = false;
      for (double &bin : bins) {
        if (bin + item <= 1.0) {
          bin += item;
          placed = true;
          break;
        }
      }
      if (!placed) bins.push_back(item);
    }
  }

  else if (name == "best_fit") {
    for (double item : items) {
      int best = -1;
      double min_remain = 2.0;
      for (int i = 0; i < bins.size(); ++i) {
        double remain = 1.0 - (bins[i] + item);
        if (remain >= 0 && remain < min_remain) {
          min_remain = remain;
          best = i;
        }
      }
      if (best == -1) bins.push_back(item);
      else bins[best] += item;
    }
  }

  else if (name == "worst_fit") {
    for (double item : items) {
      int worst = -1;
      double max_remain = -1;
      for (int i = 0; i < bins.size(); ++i) {
        double remain = 1.0 - (bins[i] + item);
        if (remain >= 0 && remain > max_remain) {
          max_remain = remain;
          worst = i;
        }
      }
      if (worst == -1) bins.push_back(item);
      else bins[worst] += item;
    }
  }

  return bins.size();
}
