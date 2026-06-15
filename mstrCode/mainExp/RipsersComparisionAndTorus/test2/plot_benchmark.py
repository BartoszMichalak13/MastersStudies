import sys
import pandas as pd
import matplotlib.pyplot as plt

if __name__ == "__main__":
    results_file = sys.argv[1]
    
    df = pd.read_csv(results_file)

    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(16, 6))

    # Plot RAM Usage
    ax1.plot(df['N'], df['PY_NO_THR_RAM'], marker='s', label='Ripser.py (No Threshold)', color='red', linewidth=2)
    ax1.plot(df['N'], df['PY_THR_RAM'], marker='*', label='Ripser.py (Threshold=2.0)', color='orange', linewidth=2)
    ax1.plot(df['N'], df['CPP_RAM'], marker='o', label='Native C++ (Threshold=2.0)', color='blue', linewidth=2)
    ax1.plot(df['N'], df['SUB_RAM'], marker='^', label='Subprocess (Py -> C++)', color='green', linewidth=2, linestyle='-.')
    ax1.plot(df['N'], df['GIOTTO_RAM'], marker='s', label='Giotto-TDA', color='red', linewidth=2, linestyle='--')

    ax1.set_title("Memory Usage vs Number of Points")
    ax1.set_xlabel("Number of Points (N)")
    ax1.set_ylabel("Peak RAM (MB)")
    ax1.grid(True, linestyle='--', alpha=0.7)
    ax1.legend()

    # Plot Time Usage
    ax2.plot(df['N'], df['PY_NO_THR_TIME'], marker='s', label='Ripser.py (No Threshold)', color='red', linewidth=2)
    ax2.plot(df['N'], df['PY_THR_TIME'], marker='*', label='Ripser.py (Threshold=2.0)', color='orange', linewidth=2)
    ax2.plot(df['N'], df['CPP_TIME'], marker='o', label='Native C++ (Threshold=2.0)', color='blue', linewidth=2)
    ax2.plot(df['N'], df['SUB_TIME'], marker='^', label='Subprocess (Py -> C++)', color='green', linewidth=2, linestyle='-.')
    ax2.plot(df['N'], df['GIOTTO_TIME'], marker='s', label='Giotto-TDA', color='red', linewidth=2, linestyle='--')

    ax2.set_title("Execution Time vs Number of Points")
    ax2.set_xlabel("Number of Points (N)")
    ax2.set_ylabel("Time (seconds)")
    ax2.grid(True, linestyle='--', alpha=0.7)
    ax2.legend()

    plt.suptitle("Scalability Benchmark: Impact of Threshold Parameter", fontsize=16)
    plt.tight_layout()
    plt.savefig("../plots/benchmark_plot_giotto.png", bbox_inches='tight')
    print("Benchmark plot saved as 'benchmark_plot.png'.")