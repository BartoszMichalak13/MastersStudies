import sys
import os
import numpy as np
import matplotlib.pyplot as plt
from persim import plot_diagrams
from persim.landscapes import PersLandscapeApprox, plot_landscape_simple

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

def standard_plot_landscapes_persim(dgms, ax, title, target_dim=1, max_k=4):
    if target_dim >= len(dgms):
        ax.set_title(f"{title} ($H_{target_dim}$) - Dimension not computed")
        ax.axis('off')
        return

    dgm_for_deg = dgms[target_dim]

    finite_dgm = dgm_for_deg[dgm_for_deg[:, 1] != np.inf]

    if len(finite_dgm) > 0:
        pla = PersLandscapeApprox(dgms=[finite_dgm], hom_deg=0)

        plot_landscape_simple(
            pla,
            ax=ax,
            alpha=0.5,
            title=f'{title} ($H_{target_dim}$)',
            depth_range=range(max_k)
        )
    else:
        ax.set_title(f"{title} ($H_{target_dim}$) - No finite features")
        ax.axis('off')

if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("Usage: python cppRipserPlot.py <cpp_log_file> <max_dim>")
        sys.exit(1)

    cpp_log_file = sys.argv[1]
    max_dim = int(sys.argv[2])

    print(f"[Python] Parsing C++ Ripser output from {cpp_log_file} (maxdim={max_dim})...")
    
    os.makedirs("../../synthetic/plots/torus", exist_ok=True)

    dgms = parse_cpp_ripser_output(cpp_log_file, max_dim)

    fig, ax = plt.subplots(figsize=(8, 8))
    
    safe_dgms = []
    for d in range(len(dgms)):
        safe_dgm = np.copy(dgms[d])
        if len(safe_dgm) == 0:
            safe_dgm = np.array([[0.0, 0.0001], [0.0, np.inf]])
        safe_dgms.append(safe_dgm)
            
    plot_diagrams(safe_dgms, show=False, ax=ax)
    ax.set_title("Persistence Diagram")
    plt.savefig("../../synthetic/plots/torus/Torus_Diagram.png", bbox_inches='tight', dpi=150)
    plt.close()

    fig, ax = plt.subplots(figsize=(10, 6))
    standard_plot_barcodes(dgms, ax, "Persistence Barcodes")
    plt.savefig("../../synthetic/plots/torus/Torus_Barcode.png", bbox_inches='tight', dpi=150)
    plt.close()

    for d in range(0, max_dim + 1):
        fig, ax = plt.subplots(figsize=(10, 6))
        standard_plot_landscapes_persim(dgms, ax, "Persistence Landscape", target_dim=d, max_k=4)
        plt.savefig(f"../../synthetic/plots/torus/Torus_Landscape_H{d}.png", bbox_inches='tight', dpi=150)
        plt.close()

