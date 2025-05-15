#include "distributions.hpp"
#include <cmath>
#include <random>
#include <numeric>

std::vector<double> uniform_dist(int n) {
    return std::vector<double>(n, 1.0 / n);
}

std::vector<double> harmonic_dist(int n) {
    double Hn = 0.0;
    for (int i = 1; i <= n; ++i) Hn += 1.0 / i;
    std::vector<double> dist(n);
    for (int i = 1; i <= n; ++i) dist[i - 1] = 1.0 / (i * Hn);
    return dist;
}

std::vector<double> double_harmonic_dist(int n) {
    double Hn = 0.0;
    for (int i = 1; i <= n; ++i) Hn += 1.0 / (i * i);
    std::vector<double> dist(n);
    for (int i = 1; i <= n; ++i) dist[i - 1] = 1.0 / (i * i * Hn);
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

std::vector<int> generate_requests(int n, const std::vector<double>& dist, int length) {
    std::random_device rd;
    std::mt19937 gen(rd());
    std::discrete_distribution<> d(dist.begin(), dist.end());
    std::vector<int> result;
    for (int i = 0; i < length; ++i) {
        result.push_back(d(gen) + 1); // strony od 1 do n
    }
    return result;
}
