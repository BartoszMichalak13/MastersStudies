#include <iostream>
#include <vector>
#include <cmath>
#include <random>
#include <cstdlib>

int main(int argc, char* argv[]) {
  int n = 3;
  int num_points = 250;
  int r = 0;

  if (argc > 1) {
    num_points = std::atoi(argv[1]);
    if (argc > 2) {
      n = std::atoi(argv[2]);
      if (argc > 3) {
        r = std::atoi(argv[3]);
      }
    }
  }

  std::random_device rd;
  std::mt19937 gen(rd());
  std::uniform_real_distribution<> dis(0.0, 2 * M_PI);

  for (int p = 0; p < num_points; ++p) {
    std::vector<double> angles(n);
    for (int i = 0; i < n; ++i) {
      angles[i] = dis(gen);
    }

    for (int i = 0; i < n; ++i) {
      if (r > 0) {
        r += 1;
      std::cout << r * std::cos(angles[i]) << "," << r * std::sin(angles[i]);
      } else { // r = 1
        std::cout << std::cos(angles[i]) << "," << std::sin(angles[i]);
      }

      if (i < n - 1)
        std::cout << ",";
    }
    std::cout << "\n";
  }
  return 0;
}