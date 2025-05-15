#include "cache_algorithms.hpp"
#include <unordered_set>
#include <unordered_map>
#include <queue>
#include <vector>
#include <list>
#include <random>
#include <limits.h>

double simulate_fifo(const std::vector<int>& requests, int k) {
    std::unordered_set<int> cache;
    std::queue<int> order;
    int faults = 0;

    for (int page : requests) {
        if (cache.find(page) == cache.end()) {
            ++faults;
            if ((int)cache.size() >= k) {
                cache.erase(order.front());
                order.pop();
            }
            cache.insert(page);
            order.push(page);
        }
    }
    return (double)faults / requests.size();
}

double simulate_fwf(const std::vector<int>& requests, int k) {
    std::unordered_set<int> cache;
    int faults = 0;

    for (int page : requests) {
        if (cache.find(page) == cache.end()) {
            ++faults;
            if ((int)cache.size() >= k) {
                cache.clear(); // flush
            }
            cache.insert(page);
        }
    }
    return (double)faults / requests.size();
}

double simulate_rand(const std::vector<int>& requests, int k) {
    std::unordered_set<int> cache;
    std::vector<int> cache_vec;
    std::random_device rd;
    std::mt19937 gen(rd());
    int faults = 0;

    for (int page : requests) {
        if (cache.find(page) == cache.end()) {
            ++faults;
            if ((int)cache.size() >= k) {
                std::uniform_int_distribution<> dist(0, cache_vec.size() - 1);
                int idx = dist(gen);
                cache.erase(cache_vec[idx]);
                cache_vec[idx] = page;
            } else {
                cache_vec.push_back(page);
            }
            cache.insert(page);
        }
    }
    return (double)faults / requests.size();
}

double simulate_lru(const std::vector<int>& requests, int k) {
  std::unordered_set<int> cache;
  std::list<int> recent;
  std::unordered_map<int, std::list<int>::iterator> positions;

  int misses = 0;

  for (int page : requests) {
      if (cache.find(page) == cache.end()) {
          misses++;
          if ((int)cache.size() == k) {
              int lru = recent.back();
              recent.pop_back();
              cache.erase(lru);
              positions.erase(lru);
          }
          cache.insert(page);
      } else {
          recent.erase(positions[page]);
      }
      recent.push_front(page);
      positions[page] = recent.begin();
  }

  return (double)misses / requests.size();
}

double simulate_lfu(const std::vector<int>& requests, int k) {
  std::unordered_map<int, int> frequency;
  std::unordered_set<int> cache;

  int misses = 0;

  for (int page : requests) {
      frequency[page]++;
      if (cache.find(page) != cache.end()) {
          continue; // hit
      }

      misses++;

      if ((int)cache.size() == k) {
          // znajdź stronę z najmniejszą liczbą użyć
          int to_remove = -1;
          int min_freq = INT_MAX;
          for (int p : cache) {
              if (frequency[p] < min_freq || (frequency[p] == min_freq && p < to_remove)) {
                  min_freq = frequency[p];
                  to_remove = p;
              }
          }
          cache.erase(to_remove);
      }

      cache.insert(page);
  }

  return (double)misses / requests.size();
}

double simulate_rma(const std::vector<int>& requests, int k) {
  std::unordered_set<int> cache;
  std::unordered_set<int> marked;
  int misses = 0;

  for (int page : requests) {
      if (cache.find(page) != cache.end()) {
          marked.insert(page);
          continue;
      }

      misses++;

      if ((int)cache.size() == k) {
          // jeśli wszystkie oznaczone — reset
          if ((int)marked.size() == k) {
              marked.clear();
          }

          // wybierz nieoznaczoną do usunięcia
          std::vector<int> candidates;
          for (int p : cache) {
              if (marked.find(p) == marked.end()) {
                  candidates.push_back(p);
              }
          }

          int to_remove = candidates[rand() % candidates.size()];
          cache.erase(to_remove);
          marked.erase(to_remove);
      }

      cache.insert(page);
      marked.insert(page);
  }

  return (double)misses / requests.size();
}

