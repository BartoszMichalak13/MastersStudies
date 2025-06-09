#pragma once
#include <random>

inline std::mt19937& rng() {
  static std::random_device rd;
  static std::mt19937 gen(rd());
  return gen;
}

inline std::vector<int> sample(int len, const std::vector<double>& probs) {
  std::discrete_distribution<> dist(probs.begin(), probs.end());
  std::vector<int> out(len);
  for (int& x : out)
    x = dist(rng());
  return out;
}