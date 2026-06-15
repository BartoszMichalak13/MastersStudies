import sys
import time
import resource
import numpy as np
from gtda.homology import VietorisRipsPersistence

if __name__ == "__main__":
    points_file = sys.argv[1]
    dim = int(sys.argv[2])
    thresh = float(sys.argv[3]) if len(sys.argv) > 3 else None

    # Wczytanie punktów
    points = np.loadtxt(points_file, delimiter=',')
    
    # Giotto-TDA oczekuje danych o kształcie (n_samples, n_points, n_dimensions).
    # Dodajemy nowy wymiar na początku (n_samples = 1)
    points_3d = points[np.newaxis, :, :]
    
    # Przygotowanie listy wymiarów homologii, np. dla dim=2 to będzie [0, 1, 2]
    homology_dims = list(range(dim + 1))

    # Pomiar czasu i wywołanie
    start_time = time.time()
    if thresh and thresh > 0:
        vr = VietorisRipsPersistence(homology_dimensions=homology_dims, max_edge_length=thresh)
    else:
        # Wersja bez limitu - inf to domyślna wartość dla max_edge_weight
        vr = VietorisRipsPersistence(homology_dimensions=homology_dims, max_edge_length=np.inf)
        
    res = vr.fit_transform(points_3d)
    end_time = time.time()

    # Pomiar pamięci RAM
    usage = resource.getrusage(resource.RUSAGE_SELF)
    peak_ram_mb = usage.ru_maxrss / 1024.0

    # Print RAM, TIME
    print(f"{peak_ram_mb:.2f},{end_time - start_time:.4f}")