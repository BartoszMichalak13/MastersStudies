#include <iostream>
#include <vector>
#include <cmath>
#include <random>
#include <sys/resource.h>

// Funkcja mierząca zużycie pamięci (Resident Set Size) na systemach UNIX
void print_memory_usage() {
    struct rusage usage;
    getrusage(RUSAGE_SELF, &usage);
    // Na Linuxie ru_maxrss jest w kilobajtach, na macOS w bajtach. 
    // Przyjmujemy standard Linuxowy (dzielimy przez 1024 dla MB)
    std::cerr << "[C++] Szczytowe zużycie pamięci generatora: " 
              << usage.ru_maxrss / 1024.0 << " MB\n";
}

int main() {
    int n = 4;             // Wymiar torusa
    int num_points = 250;  // Liczba punktów
    
    // Inicjalizacja generatora liczb losowych
    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_real_distribution<> dis(0.0, 2 * M_PI);

    // Ripser oczekuje współrzędnych rozdzielonych spacjami
    for (int p = 0; p < num_points; ++p) {
        std::vector<double> angles(n);
        for (int i = 0; i < n; ++i) {
            angles[i] = dis(gen);
        }
        
        // Obliczanie współrzędnych w R^(2n)
        for (int i = 0; i < n; ++i) {
            std::cout << std::cos(angles[i]) << " " << std::sin(angles[i]);
            if (i < n - 1) std::cout << " ";
        }
        std::cout << "\n";
    }
    
    print_memory_usage();
    return 0;
}