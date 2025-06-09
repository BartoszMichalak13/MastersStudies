#pragma once
#include <vector>
#include <string>
#include <cmath>

class Graph {
public:
  virtual ~Graph() {}
  virtual int distance(int u, int v) const = 0;
  virtual std::string name() const = 0;
};

class Torus3D : public Graph {
  int size;
  int dim;
public:
  Torus3D(int n) : size(n), dim(cbrt(n)) {}

  int distance(int u, int v) const override {
    int x1 = u % dim, y1 = (u / dim) % dim, z1 = u / (dim * dim);
    int x2 = v % dim, y2 = (v / dim) % dim, z2 = v / (dim * dim);
    auto torus_dist = [this](int a, int b) {
      int d = std::abs(a - b);
      return std::min(d, dim - d);
    };
    return torus_dist(x1, x2) + torus_dist(y1, y2) + torus_dist(z1, z2);
  }

  std::string name() const override { return "Torus3D"; }
};

class Hypercube : public Graph {
  int dim;
public:
  Hypercube(int n) : dim(log2(n)) {}

  int distance(int u, int v) const override {
    return __builtin_popcount(u ^ v);
  }

  std::string name() const override { return "Hypercube"; }
};