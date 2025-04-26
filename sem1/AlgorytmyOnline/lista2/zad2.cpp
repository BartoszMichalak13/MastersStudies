#include <iostream>
#include <vector>
#include <queue>
#include <unordered_set>
#include <unordered_map>
#include <random>
#include <cmath>
#include <algorithm>
#include <set>
#include <map>
#include <ctime>
#include <list>
#include <string>
#include <limits>
#include <climits>

using namespace std;

using Page = int;
using Trace = vector<Page>;
Trace generate_uniform(int n, int len, mt19937 &rng) {
  uniform_int_distribution<> dist(1, n);
  Trace trace;
  for (int i = 0; i < len; ++i)
    trace.push_back(dist(rng));
  return trace;
}

Trace generate_harmonic(int n, int len, mt19937 &rng) {
  vector<double> probs(n);
  double Hn = 0.0;
  for (int i = 1; i <= n; ++i) Hn += 1.0 / i;
  for (int i = 0; i < n; ++i) probs[i] = 1.0 / ((i+1) * Hn);
  discrete_distribution<> dist(probs.begin(), probs.end());

  Trace trace;
  for (int i = 0; i < len; ++i)
    trace.push_back(dist(rng) + 1);
  return trace;
}

Trace generate_hyperharmonic(int n, int len, mt19937 &rng) {
  vector<double> probs(n);
  double Hn2 = 0.0;
  for (int i = 1; i <= n; ++i) Hn2 += 1.0 / (i * i);
  for (int i = 0; i < n; ++i) probs[i] = 1.0 / ((i+1)*(i+1)*Hn2);
  discrete_distribution<> dist(probs.begin(), probs.end());

  Trace trace;
  for (int i = 0; i < len; ++i)
    trace.push_back(dist(rng) + 1);
  return trace;
}

Trace generate_geometric(int n, int len, mt19937 &rng) {
  // Bernoulli trial approach: geometric with p = 0.5, truncated at n
  Trace trace;
  bernoulli_distribution coin(0.5);

  for (int i = 0; i < len; ++i) {
    int count = 1;
    while (count < n && coin(rng))
      ++count;
    trace.push_back(count);
  }
  return trace;
}

double simulate_FIFO(int k, const Trace& trace) {
  unordered_set<int> cache;
  queue<int> order;
  int misses = 0;

  for (int page : trace) {
    if (!cache.count(page)) {
      misses++;
      if (cache.size() == k) {
        int old = order.front(); order.pop();
        cache.erase(old);
      }
      cache.insert(page);
      order.push(page);
    }
  }
  return double(misses) / trace.size();
}

double simulate_FWF(int k, const Trace& trace) {
  unordered_set<int> cache;
  int misses = 0;

  for (int page : trace) {
    if (!cache.count(page)) {
      misses++;
      if (cache.size() == k)
        cache.clear();
      cache.insert(page);
    }
  }
  return double(misses) / trace.size();
}

double simulate_LRU(int k, const Trace& trace) {
  list<int> lru;
  unordered_map<int, list<int>::iterator> cache;
  int misses = 0;

  for (int page : trace) {
    if (!cache.count(page)) {
      misses++;
      if ((int)lru.size() == k) {
        int old = lru.back(); lru.pop_back();
        cache.erase(old);
      }
    } else {
      lru.erase(cache[page]);
    }
    lru.push_front(page);
    cache[page] = lru.begin();
  }
  return double(misses) / trace.size();
}

double simulate_LFU(int k, const Trace& trace) {
  unordered_map<int, int> freq;
  unordered_map<int, int> cache;
  int misses = 0;

  for (int page : trace) {
    freq[page]++;
    if (!cache.count(page)) {
      misses++;
      if ((int)cache.size() == k) {
        // Find LFU page
        int victim = -1, min_freq = INT_MAX;
        for (auto& [p, _] : cache) {
          if (freq[p] < min_freq) {
            victim = p;
            min_freq = freq[p];
          }
        }
        cache.erase(victim);
      }
      cache[page] = 1;
    }
  }
  return double(misses) / trace.size();
}

double simulate_RAND(int k, const Trace& trace, mt19937 &rng) {
  unordered_set<int> cache;
  vector<int> cache_vec;
  int misses = 0;

  for (int page : trace) {
    if (!cache.count(page)) {
      misses++;
      if ((int)cache.size() == k) {
        uniform_int_distribution<> dist(0, cache_vec.size() - 1);
        int idx = dist(rng);
        cache.erase(cache_vec[idx]);
        cache_vec[idx] = page;
        cache.insert(page);
      } else {
        cache.insert(page);
        cache_vec.push_back(page);
      }
    }
  }
  return double(misses) / trace.size();
}

double simulate_RMA(int k, const Trace& trace, mt19937 &rng) {
  struct Entry {
    int page;
    bool marked;
  };

  vector<Entry> cache;
  unordered_map<int, int> pos; // page -> index
  int misses = 0;

  for (int page : trace) {
    if (pos.count(page)) {
      cache[pos[page]].marked = true;
    } else {
      misses++;
      if ((int)cache.size() == k) {
        vector<int> unmarked;
        for (int i = 0; i < (int)cache.size(); ++i)
          if (!cache[i].marked) unmarked.push_back(i);

        if (unmarked.empty()) {
          for (auto &e : cache) e.marked = false;
          for (int i = 0; i < (int)cache.size(); ++i)
            unmarked.push_back(i);
        }

        uniform_int_distribution<> dist(0, unmarked.size() - 1);
        int idx = unmarked[dist(rng)];
        pos.erase(cache[idx].page);
        cache[idx] = { page, false };
        pos[page] = idx;
      } else {
        cache.push_back({ page, false });
        pos[page] = cache.size() - 1;
      }
    }
  }
  return double(misses) / trace.size();
}
int main() {
  mt19937 rng(time(0));
  int len = 10000;

  vector<int> ns = {20,30,40,50,60,70,80,90,100};

  for (int n : ns) {
    vector<int> ks;
    for (int k = n/10; k <= n/5; ++k) ks.push_back(k);

    for (int k : ks) {
      Trace trace = generate_geometric(n, len, rng);

      cout << "n=" << n << " k=" << k << "\n";
      cout << "FIFO: " << simulate_FIFO(k, trace) << "\n";
      cout << "FWF: " << simulate_FWF(k, trace) << "\n";
      cout << "LRU: " << simulate_LRU(k, trace) << "\n";
      cout << "LFU: " << simulate_LFU(k, trace) << "\n";
      cout << "RAND: " << simulate_RAND(k, trace, rng) << "\n";
      cout << "RMA: " << simulate_RMA(k, trace, rng) << "\n";
      cout << "---------------------------\n";
    }
  }

  return 0;
}
