import sys
import os
import numpy as np
import matplotlib.pyplot as plt
from persim import plot_diagrams

# Add path to the parent directory to import custom modules
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

if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("Usage: python cppRipserPlot.py <cpp_log_file> <max_dim>")
        sys.exit(1)

    cpp_log_file = sys.argv[1]
    max_dim = int(sys.argv[2])

    print(f"[Python] Parsing C++ Ripser output from {cpp_log_file} (maxdim={max_dim})...")
    
    # Extract calculated persistence intervals from C++ log
    dgms = parse_cpp_ripser_output(cpp_log_file, max_dim)

    # --- 1. Plotting Persistence Diagram ---
    print("Generating Persistence Diagram...")
    fig, ax = plt.subplots(figsize=(8, 8))
    
    # Failsafe for persim library when a dimension array is completely empty
    for d in range(len(dgms)):
        if len(dgms[d]) == 0: dgms[d] = np.array([[0.0, 0.0001], [0.0, np.inf]])
            
    plot_diagrams(dgms, show=False, ax=ax)
    ax.set_title("Persistence Diagram")
    plt.savefig("../../synthetic/plots/torus/Torus_Diagram.png", bbox_inches='tight', dpi=150)
    plt.close()

    # --- 2. Plotting Barcodes ---
    print("Generating Barcodes...")
    fig, ax = plt.subplots(figsize=(10, 6))
    plot_barcodes(dgms, ax, "Persistence Barcodes")
    plt.savefig("../../synthetic/plots/torus/Torus_Barcode.png", bbox_inches='tight', dpi=150)
    plt.close()

    # --- 3. Plotting Persistence Landscapes ---
    print("Generating Landscapes for all dimensions > 0...")
    for d in range(1, max_dim + 1):
        fig, ax = plt.subplots(figsize=(10, 6))
        plot_landscapes(dgms, ax, "Persistence Landscape", target_dim=d, max_k=4)
        plt.savefig(f"../../synthetic/plots/torus/Torus_Landscape_H{d}.png", bbox_inches='tight', dpi=150)
        plt.close()
