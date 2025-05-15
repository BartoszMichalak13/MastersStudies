#pragma once
#include <vector>

double simulate_fifo(const std::vector<int>& requests, int k);
double simulate_fwf(const std::vector<int>& requests, int k);
double simulate_lru(const std::vector<int>& requests, int k);
double simulate_lfu(const std::vector<int>& requests, int k);
double simulate_rand(const std::vector<int>& requests, int k);
double simulate_rma(const std::vector<int>& requests, int k);
