
import sys
import dionysus as d
import numpy as np
import resource

task = sys.argv[1]
file_path = sys.argv[2]
max_dim = int(sys.argv[3])
max_eps = float(sys.argv[4])

# 1. Load points
points = np.loadtxt(file_path, delimiter=",")

# 2. Build complex
f = d.fill_rips(points, max_dim + 1, max_eps)

# 3. Execute requested task
if task == "homology":
    d.homology_persistence(f, method="column")
elif task == "cohomology":
    # DODANA FLAGA: keep_cocycles=False
    d.cohomology_persistence(f, keep_cocycles=False)

# Print the absolute peak memory of this specific run
usage = resource.getrusage(resource.RUSAGE_SELF)
print(usage.ru_maxrss / 1024.0)
