#include "distributions.hpp"
#include <cmath>
#include <random>
#include <numeric>

std::vector<double> uniform_dist(int n) {
    return std::vector<double>(n, 1.0 / 10);
}

std::vector<double> harmonic_dist(int n) {
    double H10 = 0.0;
    for (int i = 1; i <= 10; ++i) H10 += 1.0 / i;
    std::vector<double> dist(n);
    for (int i = 1; i <= n; ++i) dist[i - 1] = 1.0 / (i * H10);
    return dist;
}

std::vector<double> double_harmonic_dist(int n) {
    double H10 = 0.0;
    for (int i = 1; i <= 10; ++i) H10 += 1.0 / (i * i);
    std::vector<double> dist(n);
    for (int i = 1; i <= n; ++i) dist[i - 1] = 1.0 / (i * i * H10);
    return dist;
}

std::vector<double> geometric_dist(int n) {
    std::vector<double> probs(n);
    for (int i = 1; i < n; ++i) probs[i - 1] = 1.0 / std::pow(2, i);
    probs[n - 1] = 1.0 / std::pow(2, n - 1);
    double sum = std::accumulate(probs.begin(), probs.end(), 0.0);
    for (double& p : probs) p /= sum;
    return probs;
}


std::vector<double> generate_elemets(int n, const std::vector<double>& dist, int length) {
  std::random_device rd;
  std::mt19937 gen(rd());
  std::discrete_distribution<> d(dist.begin(), dist.end());
  std::uniform_real_distribution<> uniform_dist(0.0, 1.0);
  std::vector<double> result;
  for (int i = 0; i < length;) {
    double value = uniform_dist(gen);
    for (int k = d(gen) + 1; k > 0; --k) {
      result.push_back(value);
      ++i;
      if (i == length) {
        return result; // Limit to 100 elements
      }
    }
  }
  return result;
}
