import sys
import os
import numpy as np
import matplotlib.pyplot as plt
from ripser import ripser
from persim import bottleneck, plot_diagrams
from gtda.homology import VietorisRipsPersistence
from gtda.diagrams import PersistenceLandscape

def parse_cpp_ripser_output(filepath, expected_max_dim):
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
    
    return [np.array(dgms_dict.get(i, np.empty((0,2)))) for i in range(expected_max_dim + 1)]

def parse_cpp_stats(filepath):
    max_ram_mb = 0.0
    wall_time = ""
    with open(filepath, "r") as file:
        for line in file:
            if "Maximum resident set size" in line:
                max_ram_mb = float(line.split(':')[-1].strip()) / 1024.0
            elif "Elapsed (wall clock) time" in line:
                wall_time = line.split(': ', 1)[-1].strip()
    return max_ram_mb, wall_time

def run_giotto_tda(points, max_dim):
    vr = VietorisRipsPersistence(
        homology_dimensions=list(range(max_dim + 1)),
        coeff=2,
        reduced_homology=False
    )
    diagrams_raw = vr.fit_transform([points])[0]

    dgms = []
    for d in range(max_dim + 1):
        dim_mask = diagrams_raw[:, 2] == d
        dim_pts = diagrams_raw[dim_mask][:, :2]

        valid_pts = dim_pts[dim_pts[:, 0] != dim_pts[:, 1]]
        dgms.append(valid_pts)

    return dgms

def safe_plot(dgms, ax, title):
    has_finite = any((len(dgm) > 0 and np.any(dgm[:, 1] != np.inf)) for dgm in dgms)

    if not has_finite:
        if len(dgms[0]) == 0:
            dgms[0] = np.array([[0.0, 0.0001], [0.0, np.inf]])
        else:
            dgms[0] = np.vstack([dgms[0], [0.0, 0.0001]])

    plot_diagrams(dgms, show=False, ax=ax)
    ax.set_title(title)

def standard_plot_barcodes(dgms, ax, title):
    y_base = 0
    y_ticks = []
    y_tick_labels = []

    colors = plt.colormaps['tab10'].colors

    for dim, dgm in enumerate(dgms):
        if len(dgm) == 0:
            continue

        dgm_sorted = dgm[np.argsort(dgm[:, 0])]

        finite_deaths = dgm_sorted[dgm_sorted[:, 1] != np.inf][:, 1] if len(dgm_sorted) > 0 else []
        max_death = np.max(finite_deaths) if len(finite_deaths) > 0 else 2.0
        inf_length = max(max_death * 1.2, 1.0)

        for birth, death in dgm_sorted:
            if np.isinf(death) or death > 1e10:
                ax.plot([birth, inf_length], [y_base, y_base], color=colors[dim % len(colors)], lw=2, linestyle='--')
            else:
                ax.plot([birth, death], [y_base, y_base], color=colors[dim % len(colors)], lw=2)
            y_base += 1

        y_ticks.append(y_base - len(dgm)/2)
        y_tick_labels.append(f"H{dim}")
        y_base += 2

    ax.set_yticks(y_ticks)
    ax.set_yticklabels(y_tick_labels)
    ax.set_title(title)
    ax.set_xlabel("Filtration (Epsilon)")

def standard_plot_landscapes_gtda(dgms, ax, title, target_dim=1, max_k=4):
    features = []
    for d, dgm in enumerate(dgms):
        if len(dgm) > 0 and d == target_dim:
            finite_dgm = dgm[(dgm[:, 1] != np.inf) & (dgm[:, 1] < 1e10)]
            if len(finite_dgm) > 0:
                dim_col = np.full((finite_dgm.shape[0], 1), d)
                features.append(np.hstack((finite_dgm, dim_col)))

    if not features:
        ax.set_title(f"{title} (No finite features)")
        return

    gtda_dgm = np.vstack(features)
    pl = PersistenceLandscape(n_layers=max_k)
    landscapes = pl.fit_transform([gtda_dgm])[0]

    for layer_idx in range(min(max_k, landscapes.shape[0])):
        ax.plot(landscapes[layer_idx, :], label=f"Layer {layer_idx+1}")

    ax.set_title(f"{title}")
    ax.legend()
    ax.set_xticks([])
    ax.set_xlabel("Filtration parameter (binned)")

if __name__ == "__main__":
    if len(sys.argv) < 5:
        print("Usage: python compare_ripser.py <points_file> <cpp_log> <cpp_stats> <dim>")
        sys.exit(1)

    points_file = sys.argv[1]
    cpp_log_file = sys.argv[2]
    cpp_stats_file = sys.argv[3]
    max_dim = int(sys.argv[4])

    os.makedirs("../plots", exist_ok=True)
    os.makedirs("../../synthetic/plots/torus", exist_ok=True)

    print("[Python] Loading C++ logs...")
    dgms_cpp = parse_cpp_ripser_output(cpp_log_file, max_dim)
    cpp_ram, cpp_time_str = parse_cpp_stats(cpp_stats_file)

    print("[Python] Loading point cloud...")
    points = np.loadtxt(points_file, delimiter=',')

    print(f"[Python] Running Python ripser (maxdim={max_dim})...")
    res = ripser(points, maxdim=max_dim, coeff=2)
    dgms_py = res['dgms']

    print(f"[Python] Running giotto-tda (maxdim={max_dim})...")
    dgms_gtda = run_giotto_tda(points, max_dim)

    print("\n--- SUMMARY AND DIFFERENCES ---")
    print("\nComputing Bottleneck distance between diagrams...")
    
    for d in range(max_dim + 1):
        cpp_finite = dgms_cpp[d][(dgms_cpp[d][:, 1] != np.inf) & (dgms_cpp[d][:, 1] < 1e10)] if len(dgms_cpp[d]) > 0 else np.empty((0,2))
        py_finite = dgms_py[d][(dgms_py[d][:, 1] != np.inf) & (dgms_py[d][:, 1] < 1e10)] if len(dgms_py[d]) > 0 else np.empty((0,2))
        gtda_finite = dgms_gtda[d][(dgms_gtda[d][:, 1] != np.inf) & (dgms_gtda[d][:, 1] < 1e10)] if len(dgms_gtda[d]) > 0 else np.empty((0,2))
        
        dist_cpp_py = bottleneck(cpp_finite, py_finite) if (len(cpp_finite) > 0 or len(py_finite) > 0) else 0.0
        dist_py_gtda = bottleneck(py_finite, gtda_finite) if (len(py_finite) > 0 or len(gtda_finite) > 0) else 0.0

        print(f"Dimension H_{d}:")
        print(f"  -> C++ vs Python Ripser: {dist_cpp_py:.6f}")
        print(f"  -> Python Ripser vs Giotto-TDA: {dist_py_gtda:.6f}")


    fig = plt.figure(figsize=(18, 5))
    ax1 = fig.add_subplot(1, 3, 1)
    standard_plot_barcodes(dgms_cpp, ax1, "C++ Ripser")
    ax2 = fig.add_subplot(1, 3, 2)
    standard_plot_barcodes(dgms_py, ax2, "Python Ripser")
    ax3 = fig.add_subplot(1, 3, 3)
    standard_plot_barcodes(dgms_gtda, ax3, "Giotto-TDA")

    plt.suptitle("Topology Comparison: Barcodes", fontsize=16)
    plt.tight_layout()
    plt.savefig("../plots/comparison_barcode.png", bbox_inches='tight', dpi=150)
    print("\nSaved barcodes plot to 'plots/comparison_barcode.png'.")
    plt.close('all')

    for DIM_TO_PLOT in range(max_dim + 1):
        fig = plt.figure(figsize=(18, 5))
        ax1 = fig.add_subplot(1, 3, 1)
        standard_plot_landscapes_gtda(dgms_cpp, ax1, "C++ Ripser Landscapes", target_dim=DIM_TO_PLOT)
        ax2 = fig.add_subplot(1, 3, 2)
        standard_plot_landscapes_gtda(dgms_py, ax2, "Python Ripser Landscapes", target_dim=DIM_TO_PLOT)
        ax3 = fig.add_subplot(1, 3, 3)
        standard_plot_landscapes_gtda(dgms_gtda, ax3, "Giotto-TDA Landscapes", target_dim=DIM_TO_PLOT)

        plt.suptitle(f"Topology Comparison: Landscapes H{DIM_TO_PLOT}", fontsize=16)
        plt.tight_layout()
        plt.savefig(f"../plots/comparison_landscapes_H{DIM_TO_PLOT}.png", bbox_inches='tight', dpi=150)
        print(f"Saved landscapes plot to 'plots/comparison_landscapes_H{DIM_TO_PLOT}.png'.")
        plt.close('all')

    fig = plt.figure(figsize=(18, 5))
    ax1 = fig.add_subplot(1, 3, 1)
    safe_plot(dgms_cpp, ax1, "C++ Ripser")
    ax2 = fig.add_subplot(1, 3, 2)
    safe_plot(dgms_py, ax2, "Python Ripser")
    ax3 = fig.add_subplot(1, 3, 3)
    safe_plot(dgms_gtda, ax3, "Giotto-TDA")

    plt.suptitle("Topology Comparison: Persistence Diagrams", fontsize=16)
    plt.tight_layout()
    plt.savefig("../plots/comparison_diagrams.png", bbox_inches='tight', dpi=150)
    print("Saved diagrams plot to 'plots/comparison_diagrams.png'.")
    plt.close('all')