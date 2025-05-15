#pragma once
#include <vector>

std::vector<double> uniform_dist(int n);
std::vector<double> harmonic_dist(int n);
std::vector<double> double_harmonic_dist(int n);
std::vector<double> geometric_dist(int n);
std::vector<int> generate_requests(int n, const std::vector<double>& dist, int length);
