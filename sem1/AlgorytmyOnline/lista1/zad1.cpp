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

  const int number_of_experiments = 100;
  // Perform experiments for different list sizes and distributions
  for (int n : ns) {
    for (const auto& dist : distributions) {
      for (const string& strategy : strategies) {
        double avg_cost = 0.0;
        double tmp_cost = 0.0;
        for (int j = 0; j < number_of_experiments; j++) {
          LinkedList list;
          int total_cost = 0;

          // Run `n` Access operations with random X
          for (int i = 0; i < n; ++i) {
            int x = dist.second(rd);  // Get random value using distribution
            total_cost += list.access(x, strategy);
          }

          // double avg_cost = static_cast<double>(total_cost) / n;
          tmp_cost += static_cast<double>(total_cost) / n;
          // cout << "tmp_cost " << tmp_cost << "\n";

        }
        cout << "n " << n << "\n";
        avg_cost = tmp_cost/number_of_experiments;
        output << n << "," << strategy << "," << avg_cost << "," << dist.first << "\n";
      }
    }
  }

  output.close();
  cout << "Results saved to results.csv" << endl;
  return 0;
}
