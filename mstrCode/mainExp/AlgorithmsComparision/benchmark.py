import sys
import os
import time
import subprocess
import threading
import numpy as np
import matplotlib.pyplot as plt
import psutil
import phat
import gudhi
import gc

RIPSER_CPP_EXECUTABLE = "../ripser/ripser"

# --- Narzędzie do mierzenia pamięci wewnątrz jednego procesu ---
class MemoryMonitor:
    def __init__(self):
        self.keep_measuring = True
        self.peak_memory = 0.0
        self.start_memory = 0.0
        self.overhead = 0.0

    def measure_memory(self):
        process = psutil.Process(os.getpid())
        while self.keep_measuring:
            try:
                mem_mb = process.memory_info().rss / (1024 * 1024)
                if mem_mb > self.peak_memory:
                    self.peak_memory = mem_mb
            except psutil.NoSuchProcess:
                pass
            time.sleep(0.005) # Szybkie próbkowanie

    def __enter__(self):
        self.keep_measuring = True
        process = psutil.Process(os.getpid())
        self.start_memory = process.memory_info().rss / (1024 * 1024)
        self.peak_memory = self.start_memory
        self.thread = threading.Thread(target=self.measure_memory)
        self.thread.start()
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.keep_measuring = False
        self.thread.join()
        self.overhead = max(0.0, self.peak_memory - self.start_memory)

# --- Funkcje pomocnicze ---
def measure_cpp_ripser(file_path, max_dim, max_eps):
    """Runs native Ripser."""
    cmd = [
        "/usr/bin/time", "-v",
        RIPSER_CPP_EXECUTABLE,
        "--format", "point-cloud",
        "--dim", str(max_dim),
        "--threshold", str(max_eps),
        file_path
    ]
    start_time = time.time()
    result = subprocess.run(cmd, capture_output=True, text=True)
    exec_time = time.time() - start_time

    peak_mem_mb = 0.0
    for line in result.stderr.split('\n'):
        if "Maximum resident set size" in line:
            kb = float(line.split(':')[-1].strip())
            peak_mem_mb = kb / 1024.0
            break
    return exec_time, peak_mem_mb


# # To jest chyba nie sprawiedliwe
# def build_complex_and_columns(file_path, max_dim, max_eps):
#     """
#     Zwraca zarówno krotki dla PHAT, jak i obiekt SimplexTree do natywnego testu GUDHI.
#     """
#     points = np.loadtxt(file_path, delimiter=",")
#     rips = gudhi.RipsComplex(points=points, max_edge_length=max_eps)
#     st = rips.create_simplex_tree(max_dimension=max_dim + 1)

#     filtration = list(st.get_filtration())
#     simplex_to_idx = {}
#     phat_columns = []

#     for idx, (simplex, _) in enumerate(filtration):
#         simplex_to_idx[tuple(simplex)] = idx
#         boundary = []
#         if len(simplex) > 1:
#             for i in range(len(simplex)):
#                 face = tuple(simplex[:i] + simplex[i+1:])
#                 boundary.append(simplex_to_idx[face])
#             boundary.sort()

#         # Używamy czystych krotek dla PHAT
#         phat_columns.append((len(simplex) - 1, boundary))

#     return phat_columns, st

def generate_gudhi_tree(file_path, max_dim, max_eps):
    """Czysta generacja kompleksu Vietorisa-Ripsa w GUDHI."""
    points = np.loadtxt(file_path, delimiter=",")
    rips = gudhi.RipsComplex(points=points, max_edge_length=max_eps)
    st = rips.create_simplex_tree(max_dimension=max_dim + 1)
    return st

from itertools import combinations

def translate_to_phat_columns_fast(st):
    """Zoptymalizowane tłumaczenie drzewa GUDHI na listę kolumn dla PHAT."""
    # Nie robimy list(st.get_filtration())! Zostawiamy jako generator, żeby oszczędzić RAM.
    filtration = st.get_filtration()

    simplex_to_idx = {}
    phat_columns = []

    # Mikro-optymalizacja: przypisanie metody do zmiennej lokalnej omija
    # kosztowne wyszukiwanie atrybutu .append w każdej iteracji pętli
    add_column = phat_columns.append

    for idx, (simplex, _) in enumerate(filtration):
        # Rzutujemy na krotkę tylko raz
        t_simplex = tuple(simplex)
        simplex_to_idx[t_simplex] = idx

        dim = len(t_simplex) - 1

        if dim == 0:
            add_column((0, []))
        else:
            # itertools.combinations jest zaimplementowane w czystym C.
            # Wyciągnięcie kombinacji (dim) z simpleksu da nam idealnie wszystkie jego ściany
            # bez robienia powolnego list-slicingu.
            boundary = [
                simplex_to_idx[face]
                for face in combinations(t_simplex, dim)
            ]
            boundary.sort()
            add_column((dim, boundary))

    return phat_columns

# def translate_to_phat_columns(st):
#     """Tłumaczenie drzewa GUDHI na listę kolumn dla PHAT."""
#     filtration = list(st.get_filtration())
#     simplex_to_idx = {}
#     phat_columns = []

#     for idx, (simplex, _) in enumerate(filtration):
#         simplex_to_idx[tuple(simplex)] = idx
#         boundary = []
#         if len(simplex) > 1:
#             for i in range(len(simplex)):
#                 face = tuple(simplex[:i] + simplex[i+1:])
#                 boundary.append(simplex_to_idx[face])
#             boundary.sort()

#         phat_columns.append((len(simplex) - 1, boundary))

#     return phat_columns

# --- Parametry eksperymentu ---
point_counts = [50, 100, 150, 200, 250, 300, 350, 400, 450]#, 500]#, 550, 600, 650, 700]
max_dim = 2
max_eps = 2.0

times = {'ripser': [], 'phat_translation': [], 'gudhi_native': [], 'phat_standard': [], 'phat_twist': [], 'phat_row': []}
memory = {'ripser': [], 'phat_translation': [], 'gudhi_native': [], 'phat_standard': [], 'phat_twist': [], 'phat_row': []}

print("Starting subtractive benchmark loop...\n")
os.makedirs("plots", exist_ok=True)

for N in point_counts:
    print(f"=== Test for N = {N} points ===")

    # 1. Generate Torus
    with open("torus.csv", "w") as f:
        subprocess.run(["./torus", str(N), "3"], stdout=f)

    gc.collect()
    time.sleep(0.1) # Dajemy OS czas na zrzucenie stron pamię
    # 2. Native Ripser (C++)
    t_rips, m_rips = measure_cpp_ripser("torus.csv", max_dim, max_eps)
    times['ripser'].append(t_rips)
    memory['ripser'].append(m_rips)

    # # 3. Generowanie struktury dla GUDHI i PHAT
    # with MemoryMonitor() as monitor:
    #     start_time = time.time()
    #     columns, st = build_complex_and_columns("torus.csv", max_dim, max_eps)
    #     t_comp = time.time() - start_time

    # m_comp = monitor.overhead
    # times['complex_gen'].append(t_comp)
    # memory['complex_gen'].append(m_comp)

    st = generate_gudhi_tree("torus.csv", max_dim, max_eps)

    # gc.collect()
    # time.sleep(0.1) # Dajemy OS czas na zrzucenie stron pamię
    # # 3. GUDHI - Generacja Kompleksu (Czysty czas budowy drzewa)
    # with MemoryMonitor() as monitor:
    #     st = generate_gudhi_tree("torus.csv", max_dim, max_eps)
    #     start_time = time.time()
    #     # st.compute_persistence(persistence_dim_max = max_dim)
    #     st.persistence(persistence_dim_max = max_dim)
    #     # t_gudhi_gen = time.time() - start_time
    #     t_gudhi = time.time() - start_time

    # m_gudhi = monitor.overhead
    # times['gudhi_native'].append(t_gudhi)
    # memory['gudhi_native'].append(m_gudhi)
    # memory['gudhi_native'].append(monitor.peak_memory)
    # m_gudhi_gen = monitor.overhead
    # Dodaj t_gudhi_gen i m_gudhi_gen do odpowiednich słowników

    gc.collect()
    time.sleep(0.1) # Dajemy OS czas na zrzucenie stron pamię
    # 3.5. PHAT - Tłumaczenie do formatu macierzy (Python Overhead)
    with MemoryMonitor() as monitor:
        start_time = time.time()
        columns = translate_to_phat_columns_fast(st)
        t_phat_trans = time.time() - start_time

    m_phat_trans = monitor.overhead
    times['phat_translation'].append(t_phat_trans)
    memory['phat_translation'].append(m_phat_trans)
    # memory['phat_translation'].append(monitor.peak_memory)
    # gc.collect()
    # time.sleep(0.1) # Dajemy OS czas na zrzucenie stron pamię
    # # 4. GUDHI - Algorytm Natywny (wbudowana kohomologia)
    # with MemoryMonitor() as monitor:
    #     start_time = time.time()
    #     st.compute_persistence(persistence_dim_max = max_dim)
    #     t_gudhi = time.time() - start_time

    # m_gudhi = monitor.overhead
    # times['gudhi_native'].append(t_gudhi)
    # memory['gudhi_native'].append(m_gudhi)

    # gc.collect()
    # time.sleep(0.1) # Dajemy OS czas na zrzucenie stron pamię
    # # 5. PHAT - Algorytm Standardowy (Homologie - klasyka 2005)
    # with MemoryMonitor() as monitor:
    #     boundary_matrix_row = phat.boundary_matrix(columns=columns)
    #     start_time = time.time()
    #     boundary_matrix_row.compute_persistence_pairs(reduction=phat.reductions.standard_reduction)
    #     t_row = time.time() - start_time

    # m_row = monitor.overhead
    # times['phat_row'].append(t_row)
    # memory['phat_row'].append(m_row)
    # # memory['phat_standard'].append(monitor.peak_memory)
    # # del boundary_matrix_std

    gc.collect()
    time.sleep(0.1) # Dajemy OS czas na zrzucenie stron pamię
    # 5.5. PHAT - Algorytm Standardowy (Homologie - klasyka 2005)
    with MemoryMonitor() as monitor:
        boundary_matrix_std = phat.boundary_matrix(columns=columns)
        start_time = time.time()
        boundary_matrix_std.compute_persistence_pairs(reduction=phat.reductions.row_reduction)
        t_std = time.time() - start_time

    m_std = monitor.overhead
    times['phat_standard'].append(t_std)
    memory['phat_standard'].append(m_std)
    # memory['phat_standard'].append(monitor.peak_memory)
    del boundary_matrix_std

    # gc.collect()
    # time.sleep(0.1) # Dajemy OS czas na zrzucenie stron pamię
    # # 6. PHAT - Algorytm Twist (Clearing - 2011)
    # with MemoryMonitor() as monitor:
    #     boundary_matrix_twist = phat.boundary_matrix(columns=columns)
    #     start_time = time.time()
    #     boundary_matrix_twist.compute_persistence_pairs(reduction=phat.reductions.twist_reduction)
    #     t_twist = time.time() - start_time

    # m_twist = monitor.overhead
    # times['phat_twist'].append(t_twist)
    # memory['phat_twist'].append(m_twist)
    # # memory['phat_twist'].append(monitor.peak_memory)


    gc.collect()
    time.sleep(0.1) # Dajemy OS czas na zrzucenie stron pamię
    # 3. GUDHI - Generacja Kompleksu (Czysty czas budowy drzewa)
    with MemoryMonitor() as monitor:
        # st = generate_gudhi_tree("torus.csv", max_dim, max_eps)
        start_time = time.time()
        # st.compute_persistence(persistence_dim_max = max_dim)
        st.persistence(persistence_dim_max = max_dim)
        # t_gudhi_gen = time.time() - start_time
        t_gudhi = time.time() - start_time

    m_gudhi = monitor.overhead
    times['gudhi_native'].append(t_gudhi)
    memory['gudhi_native'].append(m_gudhi)

    # Czyszczenie pamięci
    # del boundary_matrix_twist
    del columns
    del st

    print(f"Ripser (C++):    {t_rips:.4f} s | Peak: {m_rips:.2f} MB")
    # print(f"Gudhi_Gen:       {t_gudhi_gen:.4f} s | Overhead: {m_gudhi_gen:.2f} MB")
    print(f"Phat_Gen:        {t_phat_trans:.4f} s | Overhead: {m_phat_trans:.2f} MB")
    print(f"GUDHI Natywnie:  {t_gudhi:.4f} s | Overhead: {m_gudhi:.2f} MB")
    print(f"PHAT Standard:   {t_std:.4f} s | Overhead: {m_std:.2f} MB")
    # print(f"PHAT Twist:      {t_twist:.4f} s | Overhead: {m_twist:.2f} MB")
    # print(f"PHAT Row:        {t_row:.4f} s | Overhead: {m_row:.2f} MB\n")

print("Generating plots...")

# --- Wykres czasu ---
plt.figure(figsize=(10, 6))
plt.plot(point_counts, times['ripser'], marker='o', label='Ripser (Native)', color='green')
plt.plot(point_counts, times['gudhi_native'], marker='D', label='GUDHI Native (Cohomology)', color='purple')
plt.plot(point_counts, times['phat_standard'], marker='s', label='PHAT Standard (Homology 2005)', color='red')
# plt.plot(point_counts, times['phat_twist'], marker='^', label='PHAT Twist (Clearing 2011)', color='blue')
# plt.plot(point_counts, times['phat_row'], marker='x', linestyle='--', label='PHAT Row (deSilva 2011)', color='orange')
plt.plot(point_counts, times['phat_translation'], marker='x', linestyle='--', label='Data Preperation for PHAT', color='gray')
plt.title("Execution Time of TDA Algorithms by Point Count")
plt.xlabel("Number of points in cloud (3D Torus)")
plt.ylabel("Execution Time (seconds)")
plt.legend()
plt.grid(True, which="both", ls="--")
plt.savefig("plots/benchmark_time.png", dpi=300)
plt.close()

# --- Wykres czasu ---
plt.figure(figsize=(10, 6))
plt.plot(point_counts, times['ripser'], marker='o', label='Ripser (Native)', color='green')
plt.plot(point_counts, times['gudhi_native'], marker='D', label='GUDHI Native (Cohomology)', color='purple')
plt.plot(point_counts, times['phat_standard'], marker='s', label='PHAT Standard (Homology 2005)', color='red')
# plt.plot(point_counts, times['phat_twist'], marker='^', label='PHAT Twist (Clearing 2011)', color='blue')
# plt.plot(point_counts, times['phat_row'], marker='x', linestyle='--', label='PHAT Row (deSilva 2011)', color='orange')
# plt.plot(point_counts, times['phat_translation'], marker='x', linestyle='--', label='Data Preperation for PHAT', color='gray')
plt.title("Execution Time of TDA Algorithms by Point Count")
plt.xlabel("Number of points in cloud (3D Torus)")
plt.ylabel("Execution Time (seconds)")
plt.legend()
plt.grid(True, which="both", ls="--")
plt.savefig("plots/benchmark_time_no_translation.png", dpi=300)
plt.close()

# --- Wykres pamięci ---
plt.figure(figsize=(10, 6))
plt.plot(point_counts, memory['ripser'], marker='o', label='Ripser Peak Memory', color='green')
plt.plot(point_counts, memory['gudhi_native'], marker='D', label='GUDHI Native Overhead', color='purple')
plt.plot(point_counts, memory['phat_standard'], marker='s', label='PHAT Standard Overhead', color='red')
# plt.plot(point_counts, memory['phat_twist'], marker='^', label='PHAT Twist Overhead', color='blue')
# plt.plot(point_counts, memory['phat_row'], marker='x', linestyle='--', label='PHAT Row (deSilva 2011)', color='orange')
plt.plot(point_counts, memory['phat_translation'], marker='x', linestyle='--', label='Data Preperation for PHAT', color='gray')
plt.title("Memory Usage (Peak/Overhead)")
plt.xlabel("Number of points in cloud (3D Torus)")
plt.ylabel("Memory Usage (MB)")
plt.legend()
plt.grid(True, which="both", ls="--")
plt.savefig("plots/benchmark_memory.png", dpi=300)
plt.close()

# --- Wykres pamięci ---
plt.figure(figsize=(10, 6))
plt.plot(point_counts, memory['ripser'], marker='o', label='Ripser Peak Memory', color='green')
plt.plot(point_counts, memory['gudhi_native'], marker='D', label='GUDHI Native Overhead', color='purple')
plt.plot(point_counts, memory['phat_standard'], marker='s', label='PHAT Standard Overhead', color='red')
# plt.plot(point_counts, memory['phat_twist'], marker='^', label='PHAT Twist Overhead', color='blue')
# plt.plot(point_counts, memory['phat_row'], marker='x', linestyle='--', label='PHAT Row (deSilva 2011)', color='orange')
# plt.plot(point_counts, memory['phat_translation'], marker='x', linestyle='--', label='Data Preperation for PHAT', color='gray')
plt.title("Memory Usage (Peak/Overhead)")
plt.xlabel("Number of points in cloud (3D Torus)")
plt.ylabel("Memory Usage (MB)")
plt.legend()
plt.grid(True, which="both", ls="--")
plt.savefig("plots/benchmark_memory_no_translation.png", dpi=300)
plt.close()

print("Done! Check your plots.")