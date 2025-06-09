#pragma once
#include "../utils/rng.hpp"
#include <vector>
#include <string>
#include <numeric>
#include <cmath>

class Distribution {
public:
  virtual ~Distribution() = default;
  Distribution(int n) : n(n) {};
  virtual std::vector<int> generate(int len) const = 0; // Generate a vector of samples
  virtual std::string name() const = 0; // Return the name of the distribution

protected:
  int n; // Number of elements in the distribution
};

class HarmonicDistribution : public Distribution {
public:
  HarmonicDistribution(int n) : Distribution(n) {}

  std::vector<int> generate(int len) const override {
    std::vector<double> probs(n);
    double Hn = 0;
    for (int i = 1; i <= n; ++i)
      Hn += 1.0 / i;
    for (int i = 0; i < n; ++i)
      probs[i] = (1.0 / (i + 1)) / Hn;
    return sample(len, probs);
  }

  std::string name() const override { return "Harmonic"; }
};

class UniformDistribution : public Distribution {
public:
  UniformDistribution(int n) : Distribution(n) {}

  std::vector<int> generate(int len) const override {
    std::vector<int> out(len);
    std::uniform_int_distribution<int> dist(0, n - 1); // Proper uniform dist
    auto& gen = rng();
    for (int& x : out)
      x = dist(gen); // Use the distribution to sample from rng

    return out;
  }

  std::string name() const override { return "Uniform"; }
};

class DoubleHarmonicDistribution : public Distribution {
public:
  DoubleHarmonicDistribution(int n) : Distribution(n) {}

  std::vector<int> generate(int len) const override {
    std::vector<double> probs(n);
    double Hn2 = 0;
    for (int i = 1; i <= n; ++i)
      Hn2 += 1.0 / (i * i);
    for (int i = 0; i < n; ++i)
      probs[i] = (1.0 / ((i + 1) * (i + 1))) / Hn2;
    return sample(len, probs);
  }

  std::string name() const override { return "DoubleHarmonic"; }
};