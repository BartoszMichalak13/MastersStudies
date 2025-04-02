#include <iostream>
#include <fstream>
#include <random>
#include <vector>
#include <string>
#include <cmath>
#include <algorithm>

using namespace std;

// Singly Linked List Node
struct Node {
    int value;
    int count;  // For count strategy
    Node* next;

    Node(int v) : value(v), count(0), next(nullptr) {}
};

// Singly Linked List
class LinkedList {
public:
  LinkedList() : head(nullptr), size(0) {}

  // Access(i) operation
  int access(int i, const string& strategy) {
    int cost = 0;
    Node* prev = nullptr;
    Node* curr = head;

    // Traverse the list to find the element
    while (curr) {
      cost++;
      if (curr->value == i) {
        // Found the element
        apply_strategy(curr, prev, strategy);
        return cost;
      }
      prev = curr;
      curr = curr->next;
    }

    // Element not found, insert at the end
    if (cost == size) {
      insert_at_end(i);
    }
    apply_strategy(curr, prev, strategy);
    return cost;
  }

  // Insert element at the end of the list
  void insert_at_end(int i) {
    Node* new_node = new Node(i);
    if (!head) {
      head = new_node;
    } else {
      Node* temp = head;
      while (temp->next) {
        temp = temp->next;
      }
      temp->next = new_node;
    }
    size++;
  }

  // Apply self-organizing strategy
  void apply_strategy(Node* curr, Node* prev, const string& strategy) {
    if (strategy == "MoveToFront") {
      move_to_front(curr);
    } else if (strategy == "Transpose") {
      transpose(curr, prev);
    } else if (strategy == "Count") {
      count(curr);
    }
  }

  // Move the found element to the front
  void move_to_front(Node* curr) {
    // If the element is already at the front, do nothing
    if (curr == head) return;

    // Find the previous node
    Node* prev = nullptr;
    Node* temp = head;
    while (temp != curr) {
      prev = temp;
      temp = temp->next;
    }

    // Now `prev` points to the node before `curr`
    if (prev != nullptr && curr != nullptr) {
      // Disconnect the node from its current position
      prev->next = curr->next;

      // Move `curr` to the front
      curr->next = head;
      head = curr;
    }
  }

  // Transpose the found element with the one in front
  void transpose(Node* curr, Node* prev) {
    if (!prev || !curr) return;
    swap(curr->value, prev->value);
  }

  // Increment the count of the element for the Count strategy
  void count(Node* curr)
  {
    if (curr == nullptr) return; // Add a check to ensure `curr` is not null
    curr->count++;  // Increment the count for the element
    sort_list_by_count();
  }
  // Sort the list by the count (descending order)
  void sort_list_by_count()
  {
    vector<Node*> nodes;
    Node* temp = head;
    while (temp) {
      nodes.push_back(temp);
      temp = temp->next;
    }
    sort(nodes.begin(), nodes.end(), [](Node* a, Node* b) {
      return a->count > b->count;
    });

    for (size_t i = 0; i < nodes.size() - 1; ++i) {
      nodes[i]->next = nodes[i + 1];
    }
    nodes.back()->next = nullptr;
    head = nodes[0];
  }

  int get_size() const { return size; }

private:
  Node* head;
  int size;
};

// Distribution Functions
int uniform_distribution(random_device& rd) {
  static mt19937 gen(rd());
  uniform_int_distribution<> dist(1, 100);
  return dist(gen);
}

int harmonic_distribution(random_device& rd) {
  static mt19937 gen(rd());
  static vector<double> probabilities(100);
  static bool initialized = false;

  if (!initialized) {
    double H100 = 0.0;
    for (int i = 1; i <= 100; ++i) {
      H100 += 1.0 / i;
    }

    for (int i = 1; i <= 100; ++i) {
      probabilities[i - 1] = (1.0 / (i * H100));// / H100;
    }
    initialized = true;
  }

  discrete_distribution<int> dist(probabilities.begin(), probabilities.end());
  return dist(gen) + 1;
}

int biharmonic_distribution(random_device& rd) {
  static mt19937 gen(rd());
  static vector<double> probabilities(100);
  static bool initialized = false;

  if (!initialized) {
    double H100 = 0.0;
    for (int i = 1; i <= 100; ++i) {
      H100 += 1.0 / (i * i);
    }

    for (int i = 1; i <= 100; ++i) {
      probabilities[i - 1] = (1.0 / (i * i * H100));// / H100;
    }
    initialized = true;
  }

  discrete_distribution<int> dist(probabilities.begin(), probabilities.end());
  return dist(gen) + 1;
}

int custom_geometric_distribution(random_device& rd) {
  static mt19937 gen(rd());
  static vector<double> probabilities(100);
  static bool initialized = false;

  if (!initialized) {
    for (int i = 1; i < 100; i++) {
      probabilities[i - 1] = (1.0 / (1 << i));
      // cout << probabilities[i - 1] << endl;
    }
    probabilities[99] = (1.0 / (1 << 99));
    initialized = true;
  }

  discrete_distribution<int> dist(probabilities.begin(), probabilities.end());
  return dist(gen) + 1;
}

int geo(random_device& rd) {
  static mt19937 gen(rd());
  static geometric_distribution<int> dist(0.5);
  return dist(gen);
}

// Main function
int main() {
  ofstream output("results.csv");
  output << "n,Strategy,AvgCost,Distribution\n";

  vector<pair<string, int(*)(random_device&)>> distributions = {
    {"Uniform", uniform_distribution},
    {"Harmonic", harmonic_distribution},
    {"Biharmonic", biharmonic_distribution},
    // {"Geometric", custom_geometric_distribution}
    {"Geometric", geo}
  };

  random_device rd;
  vector<int> ns = {100, 500, 1000, 5000, 10000, 50000, 100000};
  vector<string> strategies = {"NoReorder", "MoveToFront", "Transpose", "Count"};

  // Perform experiments for different list sizes and distributions
  for (int n : ns) {
    for (const auto& dist : distributions) {
      for (const string& strategy : strategies) {
        LinkedList list;
        int total_cost = 0;

        // Run `n` Access operations with random X
        for (int i = 0; i < n; ++i) {
          int x = dist.second(rd);  // Get random value using distribution
          total_cost += list.access(x, strategy);
        }

        double avg_cost = static_cast<double>(total_cost) / n;
        output << n << "," << strategy << "," << avg_cost << "," << dist.first << "\n";
      }
    }
  }

  output.close();
  cout << "Results saved to results.csv" << endl;
  return 0;
}


// #include <iostream>
// #include <fstream>
// #include <random>
// #include <vector>
// #include <string>

// using namespace std;

// int uniform_distribution(random_device &rd) {
//     static mt19937 gen(rd());
//     uniform_int_distribution<> dist(1, 100);
//     return dist(gen);
// }

// int harmonic_distribution(random_device &rd) {
//     static mt19937 gen(rd());
//     static vector<double> probabilities(100);
//     static bool initialized = false;

//     if (!initialized) {
//         double H100 = 0.0;
//         for (int i = 1; i <= 100; ++i) {
//             H100 += 1.0 / i;
//         }

//         for (int i = 1; i <= 100; ++i) {
//             probabilities[i - 1] = (1.0 / i) / H100;
//         }
//         initialized = true;
//     }

//     discrete_distribution<int> dist(probabilities.begin(), probabilities.end());
//     return dist(gen) + 1;
// }

// int biharmonic_distribution(random_device &rd) {
//     static mt19937 gen(rd());
//     static vector<double> probabilities(100);
//     static bool initialized = false;

//     if (!initialized) {
//         double H100 = 0.0;
//         for (int i = 1; i <= 100; ++i) {
//             H100 += 1.0 / (i * i);
//         }

//         for (int i = 1; i <= 100; ++i) {
//             probabilities[i - 1] = (1.0 / (i * i)) / H100;
//         }
//         initialized = true;
//     }

//     discrete_distribution<int> dist(probabilities.begin(), probabilities.end());
//     return dist(gen) + 1;
// }

// int custom_geometric_distribution(random_device &rd) {
//     static mt19937 gen(rd());
//     static vector<double> probabilities(100);
//     static bool initialized = false;

//     if (!initialized) {
//         for (int i = 1; i < 100; i++) probabilities[i - 1] = (1.0 / (1 << i));
//         probabilities[99] = 1.0 / 299;
//         initialized = true;
//     }

//     discrete_distribution<int> dist(probabilities.begin(), probabilities.end());
//     return dist(gen) + 1;
// }

// int main() {
//     ofstream output("results.csv");
//     output << "n,Strategy,AvgCost,Distribution\n";

//     // Example set of distributions
//     vector<pair<string, int(*)(random_device&)>> distributions = {
//         {"Uniform", uniform_distribution},
//         {"Harmonic", harmonic_distribution},
//         {"Biharmonic", biharmonic_distribution},
//         {"Geometric", custom_geometric_distribution}
//     };

//     random_device rd;

//     // Example: run the experiments for 100, 500, 1000
//     vector<int> ns = {100, 500, 1000, 5000, 10000, 50000, 100000};
//     vector<string> strategies = {"NoReorder", "MoveToFront", "Transpose"};

//     for (int n : ns) {
//         for (const auto& dist : distributions) {
//             for (const string& strategy : strategies) {
//                 // Example of how you might calculate average cost:
//                 int total_cost = 0;
//                 for (int i = 0; i < n; ++i) {
//                     total_cost += dist.second(rd);  // Run the distribution function
//                 }

//                 double avg_cost = static_cast<double>(total_cost) / n;
//                 output << n << "," << strategy << "," << avg_cost << "," << dist.first << "\n";
//             }
//         }
//     }

//     output.close();
//     cout << "Results saved to results.csv" << endl;
//     return 0;
// }











// #include <iostream>
// #include <fstream>
// #include <vector>
// #include <list>
// #include <random>
// #include <algorithm>

// using namespace std;

// // Probability Distributions
// int uniform_distribution(random_device &rd) {
//     static mt19937 gen(rd());
//     uniform_int_distribution<int> dist(1, 100);
//     return dist(gen);
// }

// int harmonic_distribution(random_device &rd) {
//     static mt19937 gen(rd());
//     static static vector<double> probabilities(100);
//     static bool initialized = false;

//     if (!initialized) {
//         double H100 = 0;
//         for (int i = 1; i <= 100; i++) H100 += 1.0 / i;
//         for (int i = 1; i <= 100; i++) probabilities[i - 1] = (1.0 / (i * H100));
//         initialized = true;
//     }

//     discrete_distribution<int> dist(probabilities.begin(), probabilities.end());
//     return dist(gen) + 1;
// }

// int biharmonic_distribution(random_device &rd) {
//     static mt19937 gen(rd());
//     static static vector<double> probabilities(100);
//     static bool initialized = false;

//     if (!initialized) {
//         double H100 = 0;
//         for (int i = 1; i <= 100; i++) H100 += 1.0 / (i * i);
//         for (int i = 1; i <= 100; i++) probabilities[i - 1] = (1.0 / (i * i * H100));
//         initialized = true;
//     }

//     discrete_distribution<int> dist(probabilities.begin(), probabilities.end());
//     return dist(gen) + 1;
// }

// int custom_geometric_distribution(random_device &rd) {
//     static mt19937 gen(rd());
//     static static vector<double> probabilities(100);
//     static bool initialized = false;

//     if (!initialized) {
//         for (int i = 1; i < 100; i++) probabilities[i - 1] = (1.0 / (1 << i));
//         probabilities[99] = 1.0 / 299;
//         initialized = true;
//     }

//     discrete_distribution<int> dist(probabilities.begin(), probabilities.end());
//     return dist(gen) + 1;
// }

// // Self-organizing List
// struct SelfOrganizingList {
//     list<int> lst;

//     int access_no_reorder(int i) {
//         int cost = 1;
//         for (auto it = lst.begin(); it != lst.end(); ++it, ++cost) {
//             if (*it == i) return cost;
//         }
//         lst.push_back(i);
//         return cost;
//     }

//     int access_move_to_front(int i) {
//         int cost = 1;
//         for (auto it = lst.begin(); it != lst.end(); ++it, ++cost) {
//             if (*it == i) {
//                 lst.erase(it);
//                 lst.push_front(i);
//                 return cost;
//             }
//         }
//         lst.push_back(i);
//         return cost;
//     }

//     int access_transpose(int i) {
//         int cost = 1;
//         for (auto it = lst.begin(); it != lst.end(); ++it, ++cost) {
//             if (*it == i) {
//                 if (it != lst.begin()) {
//                     auto prev_it = prev(it);
//                     iter_swap(prev_it, it);
//                 }
//                 return cost;
//             }
//         }
//         lst.push_back(i);
//         return cost;
//     }
// };

// void run_experiment(int n, ofstream &output, int (*distribution)(random_device &)) {
//     random_device rd;
//     vector<string> strategies = {"NoReorder", "MoveToFront", "Transpose"};

//     for (const auto &strategy : strategies) {
//         SelfOrganizingList sol;
//         long long total_cost = 0;
//         for (int j = 0; j < n; j++) {
//             int value = distribution(rd);
//             if (strategy == "NoReorder") total_cost += sol.access_no_reorder(value);
//             else if (strategy == "MoveToFront") total_cost += sol.access_move_to_front(value);
//             else if (strategy == "Transpose") total_cost += sol.access_transpose(value);
//         }
//         output << n << "," << strategy << "," << (total_cost / (double)n) << endl;
//     }
// }

// int main() {
//     vector<int> n_values = {100, 500, 1000, 5000, 10000, 50000, 100000};
//     vector<pair<string, int(*)(random_device &)>> distributions = {
//         {"Uniform", uniform_distribution},
//         {"Harmonic", harmonic_distribution},
//         {"Biharmonic", biharmonic_distribution},
//         {"Geometric", custom_geometric_distribution}
//     };

//     ofstream output("results.csv");
//     output << "n,Strategy,AvgCost,Distribution\n";

//     for (const auto &[dist_name, dist_func] : distributions) {
//         for (int n : n_values) {
//             run_experiment(n, output, dist_func);
//         }
//     }

//     output.close();
//     cout << "Results saved to results.csv" << endl;
//     return 0;
// }
