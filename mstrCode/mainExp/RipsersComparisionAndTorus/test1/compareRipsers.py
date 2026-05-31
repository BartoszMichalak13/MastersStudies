import sys
import os
import numpy as np
import matplotlib.pyplot as plt
from ripser import ripser
from persim import bottleneck, plot_diagrams

sys.path.append(os.path.abspath('../../'))

from tda import plot_barcodes, plot_landscapes

def parse_cpp_ripser_output(filepath, expected_max_dim):
    """Parses standard output from the original C++ Ripser."""
    dgms_dict = {0: []}
    current_dim = 0
    with open(filepath, "r") as file:
        for line in file:
            line = line.strip()
            if not line: continue
            if line.startswith('persistence intervals in dim'):
                current_dim = int(line.split()[-1].replace(':', ''))
                dgms_dict[current_dim] = []
            elif line.startswith('['):
                parts = line[1:-1].split(',')
                birth = float(parts[0].strip())
                death_str = parts[1].strip()
                death = float('inf') if death_str == '' else float(death_str)
                dgms_dict[current_dim].append((birth, death))
    
    # Force the list to have exactly expected_max_dim + 1 arrays
    return [np.array(dgms_dict.get(i, np.empty((0,2)))) for i in range(expected_max_dim + 1)]

def parse_cpp_stats(filepath):
    """Extracts peak memory usage and time from /usr/bin/time -v output."""
    max_ram_mb = 0.0
    wall_time = ""
    with open(filepath, "r") as file:
        for line in file:
            if "Maximum resident set size" in line:
                max_ram_mb = float(line.split(':')[-1].strip()) / 1024.0
            elif "Elapsed (wall clock) time" in line:
                wall_time = line.split(': ', 1)[-1].strip()
    return max_ram_mb, wall_time

def safe_plot(dgms, ax, title):
    """Safely plots diagrams preventing 'zero-size array' crashes in persim."""
    # Check if there are any finite points in the whole diagram
    has_finite = False
    for dgm in dgms:
        if len(dgm) > 0 and np.any(dgm[:, 1] != np.inf):
            has_finite = True
            break

    if not has_finite:
        # Inject an invisible dummy point near zero so persim axis bounds calculation doesn't crash
        if len(dgms[0]) == 0:
            dgms[0] = np.array([[0.0, 0.0001], [0.0, np.inf]])
        else:
            dgms[0] = np.vstack([dgms[0], [0.0, 0.0001]])

    plot_diagrams(dgms, show=False, ax=ax)
    ax.set_title(title)

if __name__ == "__main__":
    if len(sys.argv) < 5:
        print("Usage: python compare_ripser.py <points_file> <cpp_log> <cpp_stats> <dim>")
        sys.exit(1)

    points_file = sys.argv[1]
    cpp_log_file = sys.argv[2]
    cpp_stats_file = sys.argv[3]
    max_dim = int(sys.argv[4])

    print("[Python] Loading C++ logs...")
    dgms_cpp = parse_cpp_ripser_output(cpp_log_file, max_dim)
    cpp_ram, cpp_time_str = parse_cpp_stats(cpp_stats_file)

    print("[Python] Loading point cloud...")
    points = np.loadtxt(points_file, delimiter=',')

    print(f"[Python] Running Python ripser (maxdim={max_dim})...")
    res = ripser(points, maxdim=max_dim, coeff=2)
    dgms_py = res['dgms']

    print("\n--- SUMMARY AND DIFFERENCES ---")
    print("\nComputing Bottleneck distance (difference) between diagrams...")
    
    distances = []
    for d in range(max_dim + 1):
        cpp_finite = dgms_cpp[d][dgms_cpp[d][:, 1] != np.inf] if len(dgms_cpp[d]) > 0 else np.empty((0,2))
        py_finite = dgms_py[d][dgms_py[d][:, 1] != np.inf] if len(dgms_py[d]) > 0 else np.empty((0,2))
        
        if len(cpp_finite) == 0 and len(py_finite) == 0:
            dist = 0.0
        else:
            dist = bottleneck(cpp_finite, py_finite)
        distances.append(dist)
        print(f"Dimension H_{d} - Bottleneck distance: {dist:.6f}")

    # 5. Plot the barcodes - PURE TOPOLOGY
    fig = plt.figure(figsize=(14, 6))

    ax1 = fig.add_subplot(1, 2, 1)
    plot_barcodes(dgms_cpp, ax1, "Ripser (C++) Barcodes")

    ax2 = fig.add_subplot(1, 2, 2)
    plot_barcodes(dgms_py, ax2, "Ripser.py (Python) Barcodes")

    plt.suptitle("Topology Comparison: Bottleneck Distance = 0", fontsize=16, y=1.02)
    plt.tight_layout()
    plt.savefig("../plots/comparison_barcode.png", bbox_inches='tight', dpi=150)
    print("\nSaved barcodes plot to 'plots/comparison_barcode.png'.")

    # Single Case
    fig = plt.figure(figsize=(10, 6))
    ax = fig.add_subplot(1, 1, 1)
    plot_barcodes(dgms_cpp, ax, "Ripser Barcodes")
    plt.tight_layout()
    plt.savefig("../../synthetic/plots/torus/Barcode.png", bbox_inches='tight', dpi=150)
    print("\nSaved single barcode plot to 'plots/Barcode.png'.")
    plt.close('all')


    # 6. Plot the landscapes - PURE TOPOLOGY
    # DIM_TO_PLOT = 1
    for DIM_TO_PLOT in range(max_dim + 1):
        # print(f"H_{d} - Bottleneck distance: {distances[d]:.6f}")
        fig = plt.figure(figsize=(14, 6))

        ax1 = fig.add_subplot(1, 2, 1)
        plot_landscapes(dgms_cpp, ax1, "Ripser (C++) Landscapes", target_dim=DIM_TO_PLOT, max_k=4)

        ax2 = fig.add_subplot(1, 2, 2)
        plot_landscapes(dgms_py, ax2, "Ripser.py (Python) Landscapes", target_dim=DIM_TO_PLOT, max_k=4)

        plt.suptitle("Topology Comparison: Bottleneck Distance = 0", fontsize=16, y=1.02)
        plt.tight_layout()
        plt.savefig(f"../plots/comparison_landscapes_H{DIM_TO_PLOT}.png", bbox_inches='tight', dpi=150)
        print(f"\nSaved landscapes plot to 'plots/comparison_landscapes_H{DIM_TO_PLOT}.png'.")

        # Single Case
        fig = plt.figure(figsize=(10, 6))
        ax = fig.add_subplot(1, 1, 1)

        plot_landscapes(dgms_cpp, ax, "Ripser Landscapes", target_dim=DIM_TO_PLOT, max_k=4)

        plt.tight_layout()
        plt.savefig(f"../../synthetic/plots/torus/Landscapes_H{DIM_TO_PLOT}.png", bbox_inches='tight', dpi=150)
        print(f"\nSaved single landscape plot to 'plots/Landscapes_H{DIM_TO_PLOT}.png'.")
        plt.close('all')



    # 7. Plot the original diagrams using persim (for reference)
    fig = plt.figure(figsize=(14, 6))
    ax1 = fig.add_subplot(1, 2, 1)
    safe_plot(dgms_cpp, ax1, f"Ripser (C++) Diagram")
    ax2 = fig.add_subplot(1, 2, 2)
    safe_plot(dgms_py, ax2, f"Ripser.py (Python) Diagram")
    plt.suptitle("Topology Comparison: Bottleneck Distance = 0", fontsize=16, y=1.02)
    plt.tight_layout()
    plt.savefig("../plots/comparison_diagrams.png", bbox_inches='tight', dpi=150)
    print("\nSaved diagrams plot to 'plots/comparison_diagrams.png'.")


    # Single Case
    fig = plt.figure(figsize=(10, 6))
    ax = fig.add_subplot(1, 1, 1)
    safe_plot(dgms_cpp, ax, "Ripser Diagram")
    plt.tight_layout()
    plt.savefig("../../synthetic/plots/torus/Diagram.png", bbox_inches='tight', dpi=150)
    print("\nSaved single diagram plot to 'plots/Diagram.png'.")
    plt.close('all')