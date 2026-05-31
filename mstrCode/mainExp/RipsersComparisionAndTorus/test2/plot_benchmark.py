import sys
import pandas as pd
import matplotlib.pyplot as plt

if __name__ == "__main__":
    results_file = sys.argv[1]
    
    # Read the CSV generated Bash loop
    df = pd.read_csv(results_file)

    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(14, 6))

    # Plot RAM Usage
    ax1.plot(df['N'], df['CPP_RAM'], marker='o', label='Ripser (C++) RAM', color='blue', linewidth=2)
    ax1.plot(df['N'], df['PY_RAM'], marker='s', label='Ripser.py (Python) RAM', color='orange', linewidth=2)
    ax1.set_title("Memory Usage vs Number of Points")
    ax1.set_xlabel("Number of Points (N)")
    ax1.set_ylabel("Peak RAM (MB)")
    ax1.grid(True, linestyle='--', alpha=0.7)
    ax1.legend()

    # Plot Time Usage
    ax2.plot(df['N'], df['CPP_TIME'], marker='o', label='Ripser (C++) Time', color='blue', linewidth=2)
    ax2.plot(df['N'], df['PY_TIME'], marker='s', label='Ripser.py (Python) Time', color='orange', linewidth=2)
    ax2.set_title("Execution Time vs Number of Points")
    ax2.set_xlabel("Number of Points (N)")
    ax2.set_ylabel("Time (seconds)")
    ax2.grid(True, linestyle='--', alpha=0.7)
    ax2.legend()

    plt.suptitle("Scalability Benchmark: Ripser (C++) vs Ripser.py (Python)", fontsize=16)
    plt.tight_layout()
    plt.savefig("../plots/benchmark_plot.png", bbox_inches='tight')
    print("Benchmark plot saved as 'benchmark_plot.png'.")