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

RIPSER_CPP_EXECUTABLE = "../ripser/ripser"

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

def measure_cpp_ripser(file_path, max_dim, max_eps):
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

def generate_gudhi_tree(file_path, max_dim, max_eps):
    points = np.loadtxt(file_path, delimiter=",")
    rips = gudhi.RipsComplex(points=points, max_edge_length=max_eps)
    st = rips.create_simplex_tree(max_dimension=max_dim + 1)
    return st

if __name__ == "__main__":
    point_counts = [50, 100, 150, 200, 250, 300, 350, 400, 450, 500]
    max_dim = 2
    max_eps = 2.0

    times = {'ripser': [], 'gudhi_native': []}
    memory = {'ripser': [], 'gudhi_native': []}

    print("Starting COHOMOLOGY benchmark loop...\n")

    for N in point_counts:
        print(f"=== Test for N = {N} points ===")
        with open("torus.csv", "w") as f:
            subprocess.run(["./torus", str(N), "3"], stdout=f)

        gc.collect()
        time.sleep(0.1)

        t_rips, m_rips = measure_cpp_ripser("torus.csv", max_dim, max_eps)
        times['ripser'].append(t_rips)
        memory['ripser'].append(m_rips)

        st = generate_gudhi_tree("torus.csv", max_dim, max_eps)
        gc.collect()
        time.sleep(0.1)

        with MemoryMonitor() as monitor:
            start_time = time.time()
            st.persistence(persistence_dim_max=max_dim)
            t_gudhi = time.time() - start_time

        m_gudhi = monitor.overhead
        times['gudhi_native'].append(t_gudhi)
        memory['gudhi_native'].append(m_gudhi)

        del st
        
        print(f"Ripser (C++):    {t_rips:.4f} s | Peak: {m_rips:.2f} MB")
        print(f"GUDHI Natywnie:  {t_gudhi:.4f} s | Overhead: {m_gudhi:.2f} MB\n")

    with open('results_cohom.json', 'w') as f:
        json.dump({'point_counts': point_counts, 'times': times, 'memory': memory}, f)