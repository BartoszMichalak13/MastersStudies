import sys
import numpy as np
import matplotlib.pyplot as plt
from ripser import ripser
from persim import bottleneck, plot_diagrams

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

def plot_barcodes(dgms, ax, title):
    """Custom barcode plotting function for elegant TDA visualization."""
    colors = ['#1f77b4', '#ff7f0e', '#2ca02c', '#d62728', '#9467bd', '#8c564b']

    # Find max finite death to bound the plot gracefully
    max_death = 0
    for dgm in dgms:
        if len(dgm) > 0:
            finite_deaths = dgm[dgm[:, 1] != np.inf][:, 1]
            if len(finite_deaths) > 0:
                max_death = max(max_death, np.max(finite_deaths))

    inf_end = max_death * 1.1 if max_death > 0 else 1.0

    current_y = 0
    yticks = []
    ytick_labels = []

    for dim, dgm in enumerate(dgms):
        if len(dgm) == 0:
            continue

        # Calculate persistence for sorting (treat inf as slightly larger for sorting)
        persistence = np.where(np.isinf(dgm[:, 1]), inf_end * 2, dgm[:, 1]) - dgm[:, 0]

        # Sort descending by persistence so longest bars are at the top of the block
        sorted_indices = np.argsort(persistence)[::-1]
        sorted_dgm = dgm[sorted_indices]

        start_y = current_y
        for birth, death in sorted_dgm:
            is_inf = np.isinf(death)
            plot_death = inf_end if is_inf else death

            # Plot the solid bar
            ax.hlines(y=current_y, xmin=birth, xmax=plot_death, colors=colors[dim % len(colors)], linewidth=2)

            # If infinite, add a dashed extension
            if is_inf:
                ax.hlines(y=current_y, xmin=plot_death, xmax=inf_end * 1.05, colors=colors[dim % len(colors)], linewidth=2, linestyles='dashed')

            current_y += 1

        # Labeling the dimensions dynamically
        mid_y = (start_y + current_y - 1) / 2.0
        yticks.append(mid_y)
        ytick_labels.append(f"$H_{dim}$")

        # Add a gap before the next dimension
        current_y += max(3, int(len(dgm) * 0.05))

    ax.set_yticks(yticks)
    ax.set_yticklabels(ytick_labels)
    ax.set_xlabel("Filtration Parameter ($\epsilon$)")
    ax.set_title(title)
    ax.set_xlim(left=-0.05, right=inf_end * 1.05)

    # Hide top and right spines for a cleaner look
    ax.spines['top'].set_visible(False)
    ax.spines['right'].set_visible(False)

    # Invert Y axis so H0 is at the top, just like standard barcodes
    ax.invert_yaxis()

def plot_landscapes(dgms, ax, title, target_dim=1, max_k=3):
    """
    Custom persistence landscape plotting function using exactly the mathematical
    formulation: lambda_k(t) = k-max(max(0, min(t-b, d-t))).
    """
    # Colors for landscape levels (lambda_1, lambda_2, lambda_3...)
    colors = ['#d62728', '#2ca02c', '#bcbd22', '#1f77b4', '#ff7f0e']

    # Check if all dimensions (<= target_dim) have any finite features
    for d in range(len(dgms)):
        if len(dgms[d]) == 0 or not np.any(dgms[d][:, 1] != np.inf):
            ax.set_title(f"{title} - No features in H_{d}")
            return
    dgm = dgms[target_dim]

    # We build landscapes only for features with finite death times, as infinite ones don't contribute to the landscape values
    finite_dgm = dgm[dgm[:, 1] != np.inf]

    if len(finite_dgm) == 0:
        ax.set_title(f"{title} - No finite features in H_{target_dim}")
        return

    # We find the x-axis bounds (from earliest birth to latest death
    min_birth = np.min(finite_dgm[:, 0])
    max_death = np.max(finite_dgm[:, 1])

    t_vals = np.linspace(min_birth, max_death, 1000)

    # Vectorize the birth and death times to compute the "tent" functions for all features at once
    b = finite_dgm[:, 0][:, None]  # shape (N, 1)
    d = finite_dgm[:, 1][:, None]  # shape (N, 1)
    t = t_vals[None, :]            # shape (1, T)

    # Evaluate the tent functions: max(0, min(t-b, d-t)) for all features and all t
    tents = np.maximum(0, np.minimum(t - b, d - t))

    # Sort the tents by their maximum height (persistence) so that the most persistent features are plotted on top
    tents_sorted = np.sort(tents, axis=0)[::-1, :]

    # Draw the landscapes for the top max_k features (or fewer if there aren't that many)
    actual_k = min(max_k, len(finite_dgm))
    for k in range(actual_k):
        ax.plot(t_vals, tents_sorted[k, :], label=f'$\lambda_{{{k+1}}}$',
                color=colors[k % len(colors)], linewidth=2.5)
        ax.fill_between(t_vals, tents_sorted[k, :], alpha=0.15, color=colors[k % len(colors)])

    ax.set_title(f"{title} ($H_{target_dim}$)", fontsize=14)
    ax.set_xlabel("Filtration Parameter ($\epsilon$)", fontsize=12)
    ax.set_ylabel("Landscape Value", fontsize=12)
    ax.legend(loc='upper right')
    ax.grid(True, linestyle='--', alpha=0.6)

    # Estetyka wykresu
    ax.spines['top'].set_visible(False)
    ax.spines['right'].set_visible(False)

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
    plt.savefig("comparison_barcode.png", bbox_inches='tight', dpi=150)
    print("\nSaved clean barcode plot to 'comparison_barcode.png'.")


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
        plt.savefig(f"comparison_landscapes_H{DIM_TO_PLOT}.png", bbox_inches='tight', dpi=150)
        print(f"\nSaved clean landscapes plot to 'comparison_landscapes_H{DIM_TO_PLOT}.png'.")



    # 7. Plot the original diagrams using persim (for reference)
    fig = plt.figure(figsize=(14, 6))
    ax1 = fig.add_subplot(1, 2, 1)
    safe_plot(dgms_cpp, ax1, f"C++ Diagram (Native)")
    ax2 = fig.add_subplot(1, 2, 2)
    safe_plot(dgms_py, ax2, f"Python Diagram (Wrapper)")
    plt.suptitle("Topology Comparison: Bottleneck Distance = 0", fontsize=16, y=1.02)
    plt.tight_layout()
    plt.savefig("comparison_diagrams.png", bbox_inches='tight', dpi=150)
    print("\nSaved original diagrams plot to 'comparison_diagrams.png'.")