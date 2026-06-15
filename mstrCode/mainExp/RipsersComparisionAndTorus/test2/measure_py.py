import sys
import time
import resource
import numpy as np
from ripser import ripser

if __name__ == "__main__":
    points_file = sys.argv[1]
    dim = int(sys.argv[2])

    thresh = float(sys.argv[3]) if len(sys.argv) > 3 else None
    points = np.loadtxt(points_file, delimiter=',')
    
    start_time = time.time()
    if thresh and thresh > 0:
        res = ripser(points, maxdim=dim, coeff=2, thresh=thresh)
    else:
        res = ripser(points, maxdim=dim, coeff=2)
    end_time = time.time()

    usage = resource.getrusage(resource.RUSAGE_SELF)
    peak_ram_mb = usage.ru_maxrss / 1024.0

    print(f"{peak_ram_mb:.2f},{end_time - start_time:.4f}")