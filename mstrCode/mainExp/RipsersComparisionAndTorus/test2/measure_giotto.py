import sys
import time
import resource
import numpy as np
from gtda.homology import VietorisRipsPersistence

if __name__ == "__main__":
    points_file = sys.argv[1]
    dim = int(sys.argv[2])
    thresh = float(sys.argv[3]) if len(sys.argv) > 3 else None

    points = np.loadtxt(points_file, delimiter=',')
    points_3d = points[np.newaxis, :, :]
    homology_dims = list(range(dim + 1))

    start_time = time.time()
    if thresh and thresh > 0:
        vr = VietorisRipsPersistence(homology_dimensions=homology_dims, max_edge_length=thresh)
    else:
        vr = VietorisRipsPersistence(homology_dimensions=homology_dims, max_edge_length=np.inf)
        
    res = vr.fit_transform(points_3d)
    end_time = time.time()

    usage = resource.getrusage(resource.RUSAGE_SELF)
    peak_ram_mb = usage.ru_maxrss / 1024.0

    print(f"{peak_ram_mb:.2f},{end_time - start_time:.4f}")