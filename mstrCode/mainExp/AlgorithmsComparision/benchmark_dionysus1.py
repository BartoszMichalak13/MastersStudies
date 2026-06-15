# import os
# import sys
# import time
# import subprocess
# import numpy as np
# import matplotlib
# matplotlib.use('Agg')
# import matplotlib.pyplot as plt

# # Ścieżki do plików wykonywalnych C++
# RIPSER_EXE = "../ripser/ripser"  # Dostosuj tę ścieżkę, jeśli ripser jest gdzie indziej
# D1_HOMOLOGY_EXE = "../dionysus-1/build/examples/rips/rips"
# D1_COHOMOLOGY_EXE = "../dionysus-1/build/examples/cohomology/rips-cohomology"

# def measure_binary(cmd):
#     """Uruchamia proces przez /usr/bin/time -v i wyciąga realny czas oraz Peak RAM."""
#     full_cmd = ["/usr/bin/time", "-v"] + cmd

#     start_time = time.time()
#     result = subprocess.run(full_cmd, capture_output=True, text=True)
#     exec_time = time.time() - start_time

#     peak_mem_mb = 0.0
#     # Szukamy informacji o pamięci w stderr (tam /usr/bin/time wypluwa dane)
#     for line in result.stderr.split('\n'):
#         if "Maximum resident set size" in line:
#             try:
#                 kb = float(line.split(':')[-1].strip())
#                 peak_mem_mb = kb / 1024.0
#             except ValueError:
#                 pass
#             break

#     # Sprawdzenie czy program nie wywalił błędu (np. brak pliku)
#     if result.returncode != 0:
#         print(f" BŁĄD podczas uruchomienia: {' '.join(cmd)}")
#         print(result.stderr)

#     return exec_time, peak_mem_mb

# # --- Parametry testu ---
# # Uwaga: Dla czystej homologii N=700 może zmiażdżyć RAM. Zaczynamy ostrożnie.
# point_counts = [50, 100, 150, 200, 250, 300, 350, 400, 450, 500]#, 550, 600, 650, 700, 750, 800, 850, 900, 950, 1000]
# max_dim = 2
# max_eps = 2.0  # W Dionysusie to odpowiada maksymalnemu dystansowi krawędzi

# times = {'ripser': [], 'd_homology': [], 'd_cohomology': []}
# memory = {'ripser': [], 'd_homology': [], 'd_cohomology': []}

# print("Uruchamianie czystego benchmarku C++ (Dionysus 1 vs Ripser)...\n")
# os.makedirs("plots", exist_ok=True)

# for N in point_counts:
#     print(f"=== Test dla N = {N} punktów ===")

#     # 1. Generowanie Torusa (zakładamy, że generator 'torus' jest w bieżącym folderze)
#     with open("torus.csv", "w") as f:
#         subprocess.run(["./torus", str(N), "3"], stdout=f)

#     # 2. Pomiar: Ripser (C++)
#     cmd_ripser = [RIPSER_EXE, "--format", "point-cloud", "--dim", str(max_dim), "--threshold", str(max_eps), "torus.csv"]
#     t_rips, m_rips = measure_binary(cmd_ripser)
#     times['ripser'].append(t_rips)
#     memory['ripser'].append(m_rips)

#     # 3. Pomiar: Dionysus 1 - Klasyczna Homologia (C++)
#     # Składnia: rips <plik_wejściowy> <wymiar> <threshold>
#     cmd_d1_hom = [D1_HOMOLOGY_EXE, "torus.csv", str(max_dim), str(max_eps)]
#     t_hom, m_hom = measure_binary(cmd_d1_hom)
#     times['d_homology'].append(t_hom)
#     memory['d_homology'].append(m_hom)

#     # 4. Pomiar: Dionysus 1 - Kohomologia (C++)
#     # Składnia: rips-cohomology <plik_wejściowy> <wymiar> <threshold>
#     cmd_d1_coh = [D1_COHOMOLOGY_EXE, "torus.csv", str(max_dim), str(max_eps)]
#     t_coh, m_coh = measure_binary(cmd_d1_coh)
#     times['d_cohomology'].append(t_coh)
#     memory['d_cohomology'].append(m_coh)

#     print(f"Ripser:       {t_rips:.4f} s | RAM: {m_rips:.2f} MB")
#     print(f"D1_Homology:  {t_hom:.4f} s | RAM: {m_hom:.2f} MB")
#     print(f"D1_Cohomol:   {t_coh:.4f} s | RAM: {m_coh:.2f} MB\n")

# # --- Generowanie Wykresów ---
# print("Generowanie czystych wykresów...")

# # Wykres Czasu
# plt.figure(figsize=(10, 6))
# plt.plot(point_counts, times['ripser'], marker='o', label='Ripser (Cohomology)', color='green')
# plt.plot(point_counts, times['d_homology'], marker='s', label='Dionysus 1 Homology', color='red')
# plt.plot(point_counts, times['d_cohomology'], marker='^', label='Dionysus 1 Cohomology', color='blue')
# plt.title("Czas wykonania algorytmów TDA (Cisco de Silva replication)")
# plt.xlabel("Liczba punktów (3D Torus)")
# plt.ylabel("Czas (sekundy)")
# plt.legend()
# plt.grid(True, which="both", ls="--")
# plt.savefig("plots/benchmark_time.png", dpi=300)
# plt.close()

# # Wykres Pamięci RAM
# plt.figure(figsize=(10, 6))
# plt.plot(point_counts, memory['ripser'], marker='o', label='Ripser Peak RAM', color='green')
# plt.plot(point_counts, memory['d_homology'], marker='s', label='Dionysus 1 Homology RAM', color='red')
# plt.plot(point_counts, memory['d_cohomology'], marker='^', label='Dionysus 1 Cohomology RAM', color='blue')
# plt.title("Zużycie pamięci RAM (Cisco de Silva replication)")
# plt.xlabel("Liczba punktów (3D Torus)")
# plt.ylabel("Zużycie pamięci (MB)")
# plt.legend()
# plt.grid(True, which="both", ls="--")
# plt.savefig("plots/benchmark_memory.png", dpi=300)
# plt.close()

# print("Gotowe! Wykresy znajdziesz w katalogu 'plots/'.")

import os
import sys
import time
import subprocess
import numpy as np
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from scipy.spatial.distance import pdist, squareform

# Ścieżki do plików wykonywalnych C++
RIPSER_EXE = "../ripser/ripser"  # Dostosuj tę ścieżkę, jeśli ripser jest gdzie indziej
D1_HOMOLOGY_EXE = "../dionysus-1/build/examples/rips/rips"
D1_COHOMOLOGY_EXE = "../dionysus-1/build/examples/cohomology/rips-cohomology"

def measure_binary(cmd):
    """Uruchamia proces przez /usr/bin/time -v i wyciąga realny czas oraz Peak RAM."""
    full_cmd = ["/usr/bin/time", "-v"] + cmd
    
    start_time = time.time()
    result = subprocess.run(full_cmd, capture_output=True, text=True)
    exec_time = time.time() - start_time

    peak_mem_mb = 0.0
    # Szukamy informacji o pamięci w stderr (tam /usr/bin/time wypluwa dane)
    for line in result.stderr.split('\n'):
        if "Maximum resident set size" in line:
            try:
                kb = float(line.split(':')[-1].strip())
                peak_mem_mb = kb / 1024.0
            except ValueError:
                pass
            break
            
    # Sprawdzenie czy program nie wywalił błędu (np. brak pliku)
    if result.returncode != 0:
        print(f" BŁĄD podczas uruchomienia: {' '.join(cmd)}")
        print(result.stderr)
        
    return exec_time, peak_mem_mb

# --- Parametry testu ---
# Uwaga: Dla czystej homologii N=700 może zmiażdżyć RAM. Zaczynamy ostrożnie.
point_counts = [50, 100, 150, 200, 250, 300, 350, 400, 450, 500]#, 550, 600, 650, 700, 750, 800, 850, 900, 950, 1000]
max_dim = 2
max_eps = 2.0  # W Dionysusie to odpowiada maksymalnemu dystansowi krawędzi

times = {'ripser': [], 'd_homology': [], 'd_cohomology': []}
memory = {'ripser': [], 'd_homology': [], 'd_cohomology': []}

print("Uruchamianie czystego benchmarku C++ (Dionysus 1 vs Ripser)...\n")
os.makedirs("plots", exist_ok=True)

for N in point_counts:
    print(f"=== Test dla N = {N} punktów ===")

    # 1. Generowanie Torusa
    with open("torus.csv", "w") as f:
        subprocess.run(["./torus", str(N), "3"], stdout=f)

    # 2. Odczyt punktów i obliczenie macierzy odległości (Lower Triangular)
    # Wczytujemy wygenerowane punkty
    points = np.loadtxt("torus.csv", delimiter=',')

    # Liczymy pełną macierz odległości
    dist_mat = squareform(pdist(points))

    # Wyciągamy macierz dolnotrójkątną wiersz po wierszu
    # (jest to standardowy format oczekiwany przez Ripsera i Dionysusa)
    lower_triangular = []
    for i in range(len(points)):
        for j in range(i):
            lower_triangular.append(dist_mat[i, j])

    # Zapisujemy odległości do pliku
    with open("distances.csv", "w") as f:
        for d in lower_triangular:
            f.write(f"{d}\n")

    # 3. Pomiar: Ripser (C++)
    # Zmieniamy format na 'lower-distance' i podajemy distances.csv
    cmd_ripser = [RIPSER_EXE, "--format", "lower-distance", "--dim", str(max_dim), "--threshold", str(max_eps), "distances.csv"]
    t_rips, m_rips = measure_binary(cmd_ripser)
    times['ripser'].append(t_rips)
    memory['ripser'].append(m_rips)

    # 4. Pomiar: Dionysus 1 - Klasyczna Homologia (C++)
    # Podajemy distances.csv zamiast torus.csv
    cmd_d1_hom = [D1_HOMOLOGY_EXE, "distances.csv", str(max_dim), str(max_eps)]
    t_hom, m_hom = measure_binary(cmd_d1_hom)
    times['d_homology'].append(t_hom)
    memory['d_homology'].append(m_hom)

    # 5. Pomiar: Dionysus 1 - Kohomologia (C++)
    cmd_d1_coh = [D1_COHOMOLOGY_EXE, "distances.csv", str(max_dim), str(max_eps)]
    t_coh, m_coh = measure_binary(cmd_d1_coh)
    times['d_cohomology'].append(t_coh)
    memory['d_cohomology'].append(m_coh)

    print(f"Ripser:       {t_rips:.4f} s | RAM: {m_rips:.2f} MB")
    print(f"D1_Homology:  {t_hom:.4f} s | RAM: {m_hom:.2f} MB")
    print(f"D1_Cohomol:   {t_coh:.4f} s | RAM: {m_coh:.2f} MB\n")

# --- Generowanie Wykresów ---
print("Generowanie czystych wykresów...")

# Wykres Czasu
plt.figure(figsize=(10, 6))
plt.plot(point_counts, times['ripser'], marker='o', label='Ripser (Cohomology)', color='green')
plt.plot(point_counts, times['d_homology'], marker='s', label='Dionysus 1 Homology', color='red')
plt.plot(point_counts, times['d_cohomology'], marker='^', label='Dionysus 1 Cohomology', color='blue')
plt.title("Czas wykonania algorytmów TDA (Cisco de Silva replication)")
plt.xlabel("Liczba punktów (3D Torus)")
plt.ylabel("Czas (sekundy)")
plt.legend()
plt.grid(True, which="both", ls="--")
plt.savefig("plots/dio_benchmark_time.png", dpi=300)
plt.close()

# Wykres Pamięci RAM
plt.figure(figsize=(10, 6))
plt.plot(point_counts, memory['ripser'], marker='o', label='Ripser Peak RAM', color='green')
plt.plot(point_counts, memory['d_homology'], marker='s', label='Dionysus 1 Homology RAM', color='red')
plt.plot(point_counts, memory['d_cohomology'], marker='^', label='Dionysus 1 Cohomology RAM', color='blue')
plt.title("Zużycie pamięci RAM (Cisco de Silva replication)")
plt.xlabel("Liczba punktów (3D Torus)")
plt.ylabel("Zużycie pamięci (MB)")
plt.legend()
plt.grid(True, which="both", ls="--")
plt.savefig("plots/dio_benchmark_memory.png", dpi=300)
plt.close()

print("Gotowe! Wykresy znajdziesz w katalogu 'plots/'.")