# import sys
# import os
# import numpy as np
# import matplotlib.pyplot as plt
# from persim import plot_diagrams

# # Add path to the parent directory to import custom modules
# sys.path.append(os.path.abspath('../../'))
# from tda import plot_barcodes, plot_landscapes

# def parse_cpp_ripser_output(filepath, expected_max_dim):
#     """Parses standard output from the original C++ Ripser."""
#     dgms_dict = {0: []}
#     current_dim = 0
#     with open(filepath, "r") as file:
#         for line in file:
#             line = line.strip()
#             if not line: continue
#             if line.startswith('persistence intervals in dim'):
#                 current_dim = int(line.split()[-1].replace(':', ''))
#                 dgms_dict[current_dim] = []
#             elif line.startswith('['):
#                 parts = line[1:-1].split(',')
#                 birth = float(parts[0].strip())
#                 death_str = parts[1].strip()
#                 death = float('inf') if death_str == '' else float(death_str)
#                 dgms_dict[current_dim].append((birth, death))

#     # Force the list to have exactly expected_max_dim + 1 arrays
#     return [np.array(dgms_dict.get(i, np.empty((0,2)))) for i in range(expected_max_dim + 1)]

# if __name__ == "__main__":
#     if len(sys.argv) < 3:
#         print("Usage: python cppRipserPlot.py <cpp_log_file> <max_dim>")
#         sys.exit(1)

#     cpp_log_file = sys.argv[1]
#     max_dim = int(sys.argv[2])

#     print(f"[Python] Parsing C++ Ripser output from {cpp_log_file} (maxdim={max_dim})...")

#     # Extract calculated persistence intervals from C++ log
#     dgms = parse_cpp_ripser_output(cpp_log_file, max_dim)

#     # --- 1. Plotting Persistence Diagram ---
#     print("Generating Persistence Diagram...")
#     fig, ax = plt.subplots(figsize=(8, 8))

#     # Failsafe for persim library when a dimension array is completely empty
#     for d in range(len(dgms)):
#         if len(dgms[d]) == 0: dgms[d] = np.array([[0.0, 0.0001], [0.0, np.inf]])

#     plot_diagrams(dgms, show=False, ax=ax)
#     ax.set_title("Persistence Diagram")
#     plt.savefig("../../synthetic/plots/torus/Torus_Diagram.png", bbox_inches='tight', dpi=150)
#     plt.close()

#     # --- 2. Plotting Barcodes ---
#     print("Generating Barcodes...")
#     fig, ax = plt.subplots(figsize=(10, 6))
#     plot_barcodes(dgms, ax, "Persistence Barcodes")
#     plt.savefig("../../synthetic/plots/torus/Torus_Barcode.png", bbox_inches='tight', dpi=150)
#     plt.close()

#     # --- 3. Plotting Persistence Landscapes ---
#     print("Generating Landscapes for all dimensions > 0...")
#     for d in range(1, max_dim + 1):
#         fig, ax = plt.subplots(figsize=(10, 6))
#         plot_landscapes(dgms, ax, "Persistence Landscape", target_dim=d, max_k=4)
#         plt.savefig(f"../../synthetic/plots/torus/Torus_Landscape_H{d}.png", bbox_inches='tight', dpi=150)
#         plt.close()


import sys
import os
import numpy as np
import matplotlib.pyplot as plt
from persim import plot_diagrams
# from gtda.diagrams import PersistenceLandscape
from persim.landscapes import PersLandscapeApprox, plot_landscape_simple

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

def standard_plot_barcodes(dgms, ax, title):
    """Plots standard TDA barcodes using purely matplotlib."""
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

# def standard_plot_landscapes_gtda(dgms, ax, title, target_dim=1, max_k=4):
#     """Computes and plots landscapes using standard giotto-tda tools."""
#     features = []
#     for d, dgm in enumerate(dgms):
#         if len(dgm) > 0 and d == target_dim:
#             # giotto-tda requires finite values for landscapes
#             finite_dgm = dgm[(dgm[:, 1] != np.inf) & (dgm[:, 1] < 1e10)]
#             if len(finite_dgm) > 0:
#                 dim_col = np.full((finite_dgm.shape[0], 1), d)
#                 features.append(np.hstack((finite_dgm, dim_col)))

#     if not features:
#         ax.set_title(f"{title} (No finite features)")
#         return

#     gtda_dgm = np.vstack(features)
#     pl = PersistenceLandscape(n_layers=max_k)
#     landscapes = pl.fit_transform([gtda_dgm])[0]

#     for layer_idx in range(min(max_k, landscapes.shape[1])):
#         ax.plot(landscapes[layer_idx, :], label=f"Layer {layer_idx+1}")

#     ax.set_title(f"{title}")
#     ax.legend()
#     ax.set_xticks([])
#     ax.set_xlabel("Filtration parameter (binned)")

def standard_plot_landscapes_persim(dgms, ax, title, target_dim=1, max_k=4):
    """Computes and plots landscapes using Persim's PersLandscapeApprox."""
    if target_dim >= len(dgms):
        ax.set_title(f"{title} ($H_{target_dim}$) - Dimension not computed")
        ax.axis('off')
        return

    dgm_for_deg = dgms[target_dim]

    # Odfiltrowanie punktów w nieskończoności
    finite_dgm = dgm_for_deg[dgm_for_deg[:, 1] != np.inf]

    if len(finite_dgm) > 0:
        pla = PersLandscapeApprox(dgms=[finite_dgm], hom_deg=0)

        # Wywołanie natywnej funkcji z persim.landscapes
        plot_landscape_simple(
            pla,
            ax=ax,
            alpha=0.5,
            title=f'{title} ($H_{target_dim}$)',
            depth_range=range(max_k)  # Ograniczenie do top K warstw
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
    
    # Upewnienie się, że foldery docelowe istnieją
    os.makedirs("../../synthetic/plots/torus", exist_ok=True)

    dgms = parse_cpp_ripser_output(cpp_log_file, max_dim)

    # --- 1. Plotting Persistence Diagram ---
    print("Generating Persistence Diagram...")
    fig, ax = plt.subplots(figsize=(8, 8))
    
    # Failsafe for persim library (copy to avoid mutating the array for other plots)
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

    # --- 2. Plotting Barcodes ---
    print("Generating Barcodes...")
    fig, ax = plt.subplots(figsize=(10, 6))
    standard_plot_barcodes(dgms, ax, "Persistence Barcodes")
    plt.savefig("../../synthetic/plots/torus/Torus_Barcode.png", bbox_inches='tight', dpi=150)
    plt.close()

    # --- 3. Plotting Persistence Landscapes ---
    print("Generating Landscapes for all dimensions ...")
    for d in range(0, max_dim + 1):
        fig, ax = plt.subplots(figsize=(10, 6))
        # standard_plot_landscapes_gtda(dgms, ax, "Persistence Landscape", target_dim=d, max_k=4)
        standard_plot_landscapes_persim(dgms, ax, "Persistence Landscape", target_dim=d, max_k=4)
        plt.savefig(f"../../synthetic/plots/torus/Torus_Landscape_H{d}.png", bbox_inches='tight', dpi=150)
        plt.close()

    print("Done! All plots saved.")