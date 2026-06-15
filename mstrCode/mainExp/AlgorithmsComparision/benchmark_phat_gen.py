import sys
import os
import time
import subprocess
import threading
import json
import gc
import numpy as np
import psutil
import gudhi
from itertools import combinations

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
            time.sleep(0.005)

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

def generate_gudhi_tree(file_path, max_dim, max_eps):
    points = np.loadtxt(file_path, delimiter=",")
    rips = gudhi.RipsComplex(points=points, max_edge_length=max_eps)
    st = rips.create_simplex_tree(max_dimension=max_dim + 1)
    return st

def translate_to_phat_columns_fast(st):
    filtration = st.get_filtration()
    simplex_to_idx = {}
    phat_columns = []
    add_column = phat_columns.append

    for idx, (simplex, _) in enumerate(filtration):
        t_simplex = tuple(simplex)
        simplex_to_idx[t_simplex] = idx
        dim = len(t_simplex) - 1

        if dim == 0:
            add_column((0, []))
        else:
            boundary = [simplex_to_idx[face] for face in combinations(t_simplex, dim)]
            boundary.sort()
            add_column((dim, boundary))
    return phat_columns

if __name__ == "__main__":
    point_counts = [50, 100, 150, 200, 250, 300, 350, 400, 450, 500]
    max_dim = 2
    max_eps = 2.0

    times = {'phat_translation': []}
    memory = {'phat_translation': []}

    print("Starting PHAT GENERATION benchmark loop...\n")

    for N in point_counts:
        print(f"=== Test for N = {N} points ===")
        with open("torus.csv", "w") as f:
            subprocess.run(["./torus", str(N), "3"], stdout=f)

        st = generate_gudhi_tree("torus.csv", max_dim, max_eps)
        gc.collect()
        time.sleep(0.1)

        # Monitorujemy tylko samo tłumaczenie kolumn
        with MemoryMonitor() as monitor:
            start_time = time.time()
            columns = translate_to_phat_columns_fast(st)
            t_phat_trans = time.time() - start_time

        m_phat_trans = monitor.overhead
        times['phat_translation'].append(t_phat_trans)
        memory['phat_translation'].append(m_phat_trans)

        del columns
        del st

        print(f"Phat_Gen: {t_phat_trans:.4f} s | Overhead: {m_phat_trans:.2f} MB\n")

    with open('results_phat_gen.json', 'w') as f:
        json.dump({'point_counts': point_counts, 'times': times, 'memory': memory}, f)