import json
import os
import matplotlib.pyplot as plt

def main():
    print("Loading benchmark results...")
    
    # Wczytywanie wyników z 3 plików
    with open('results_cohom.json', 'r') as f:
        cohom_data = json.load(f)
        
    with open('results_phat_gen.json', 'r') as f:
        phat_gen_data = json.load(f)

    with open('results_phat_standard.json', 'r') as f:
        phat_std_data = json.load(f)

    point_counts = cohom_data['point_counts']
    
    # Łączenie słowników wyników
    times = {**cohom_data['times'], **phat_gen_data['times'], **phat_std_data['times']}
    memory = {**cohom_data['memory'], **phat_gen_data['memory'], **phat_std_data['memory']}

    print("Generating plots...")
    os.makedirs("plots", exist_ok=True)

    # --- Wykres czasu z tłumaczeniem ---
    plt.figure(figsize=(10, 6))
    plt.plot(point_counts, times['ripser'], marker='o', label='Ripser (Native)', color='green')
    plt.plot(point_counts, times['gudhi_native'], marker='D', label='GUDHI Native (Cohomology)', color='purple')
    plt.plot(point_counts, times['phat_standard'], marker='s', label='PHAT Standard (Homology 2005)', color='red')
    plt.plot(point_counts, times['phat_translation'], marker='x', linestyle='--', label='Data Preperation for PHAT', color='gray')
    plt.title("Execution Time of TDA Algorithms by Point Count")
    plt.xlabel("Number of points in cloud (3D Torus)")
    plt.ylabel("Execution Time (seconds)")
    plt.legend()
    plt.grid(True, which="both", ls="--")
    plt.savefig("plots/SPLIT_benchmark_time.png", dpi=300)
    plt.close()

    # --- Wykres czasu bez tłumaczenia ---
    plt.figure(figsize=(10, 6))
    plt.plot(point_counts, times['ripser'], marker='o', label='Ripser (Native)', color='green')
    plt.plot(point_counts, times['gudhi_native'], marker='D', label='GUDHI Native (Cohomology)', color='purple')
    plt.plot(point_counts, times['phat_standard'], marker='s', label='PHAT Standard (Homology 2005)', color='red')
    plt.title("Execution Time of TDA Algorithms by Point Count")
    plt.xlabel("Number of points in cloud (3D Torus)")
    plt.ylabel("Execution Time (seconds)")
    plt.legend()
    plt.grid(True, which="both", ls="--")
    plt.savefig("plots/SPLIT_benchmark_time_no_translation.png", dpi=300)
    plt.close()

    # --- Wykres pamięci z tłumaczeniem ---
    plt.figure(figsize=(10, 6))
    plt.plot(point_counts, memory['ripser'], marker='o', label='Ripser Peak Memory', color='green')
    plt.plot(point_counts, memory['gudhi_native'], marker='D', label='GUDHI Native Overhead', color='purple')
    plt.plot(point_counts, memory['phat_standard'], marker='s', label='PHAT Standard Overhead', color='red')
    plt.plot(point_counts, memory['phat_translation'], marker='x', linestyle='--', label='Data Preperation for PHAT', color='gray')
    plt.title("Memory Usage (Peak/Overhead)")
    plt.xlabel("Number of points in cloud (3D Torus)")
    plt.ylabel("Memory Usage (MB)")
    plt.legend()
    plt.grid(True, which="both", ls="--")
    plt.savefig("plots/SPLIT_benchmark_memory.png", dpi=300)
    plt.close()

    # --- Wykres pamięci bez tłumaczenia ---
    plt.figure(figsize=(10, 6))
    plt.plot(point_counts, memory['ripser'], marker='o', label='Ripser Peak Memory', color='green')
    plt.plot(point_counts, memory['gudhi_native'], marker='D', label='GUDHI Native Overhead', color='purple')
    plt.plot(point_counts, memory['phat_standard'], marker='s', label='PHAT Standard Overhead', color='red')
    plt.title("Memory Usage (Peak/Overhead)")
    plt.xlabel("Number of points in cloud (3D Torus)")
    plt.ylabel("Memory Usage (MB)")
    plt.legend()
    plt.grid(True, which="both", ls="--")
    plt.savefig("plots/SPLIT_benchmark_memory_no_translation.png", dpi=300)
    plt.close()

    print("Done! Check your plots directory.")

if __name__ == "__main__":
    main()