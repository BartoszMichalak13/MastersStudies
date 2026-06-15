import sys
import pandas as pd
import matplotlib.pyplot as plt

if __name__ == "__main__":
    results_file = sys.argv[1]
    df = pd.read_csv(results_file)

    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(16, 6))

    # Plot RAM Usage
    ax1.plot(df['N'], df['PY_THR_RAM'], marker='*', label='Ripser.py', color='orange', linewidth=2)
    ax1.plot(df['N'], df['CPP_RAM'], marker='o', label='Native C++', color='blue', linewidth=2)
    ax1.plot(df['N'], df['SUB_RAM'], marker='^', label='Subprocess (Py -> C++)', color='green', linewidth=2, linestyle='-.')
    ax1.plot(df['N'], df['GIOTTO_RAM'], marker='s', label='Giotto-TDA', color='red', linewidth=2, linestyle='--')

    ax1.set_title("High-Scale Memory Usage")
    ax1.set_xlabel("Number of Points (N)")
    ax1.set_ylabel("Peak RAM (MB)")
    ax1.grid(True, linestyle='--', alpha=0.7)
    ax1.legend()

    # Plot Time Usage
    ax2.plot(df['N'], df['PY_THR_TIME'], marker='*', label='Ripser.py', color='orange', linewidth=2)
    ax2.plot(df['N'], df['CPP_TIME'], marker='o', label='Native C++', color='blue', linewidth=2)
    ax2.plot(df['N'], df['SUB_TIME'], marker='^', label='Subprocess (Py -> C++)', color='green', linewidth=2, linestyle='-.')
    ax2.plot(df['N'], df['GIOTTO_TIME'], marker='s', label='Giotto-TDA', color='red', linewidth=2, linestyle='--')

    ax2.set_title("High-Scale Execution Time")
    ax2.set_xlabel("Number of Points (N)")
    ax2.set_ylabel("Time (seconds)")
    ax2.grid(True, linestyle='--', alpha=0.7)
    ax2.legend()

    plt.suptitle("TDA High-Scalability Benchmark (Optimized with Threshold)", fontsize=16)
    plt.tight_layout()
    plt.savefig("../plots/scale_benchmark_plot_giotto.png", bbox_inches='tight')
    print("Saved high-scale plot: 'scale_benchmark_plot.png'.")