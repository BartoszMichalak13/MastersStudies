#pragma once
#include <vector>

std::vector<double> uniform_dist(int n);
std::vector<double> harmonic_dist(int n);
std::vector<double> double_harmonic_dist(int n);
std::vector<double> geometric_dist(int n);
std::vector<double> generate_elemets(int n, const std::vector<double>& dist, int length);
