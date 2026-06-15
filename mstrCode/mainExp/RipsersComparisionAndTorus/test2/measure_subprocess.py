import sys
import time
import resource
import subprocess

if __name__ == "__main__":
    points_file = sys.argv[1]
    dim = int(sys.argv[2])
    ripser_bin_path = sys.argv[3]

    thresh = float(sys.argv[4]) if len(sys.argv) > 4 else None

    cmd = [ripser_bin_path, "--format", "point-cloud", "--dim", str(dim)]

    if thresh and thresh > 0:
        cmd.extend(["--threshold", str(thresh)])

    cmd.append(points_file)

    start_time = time.time()
    
    subprocess.run(
        cmd,
        stdout=subprocess.DEVNULL, 
        stderr=subprocess.DEVNULL
    )
    
    end_time = time.time()

    usage = resource.getrusage(resource.RUSAGE_CHILDREN)
    peak_ram_mb = usage.ru_maxrss / 1024.0

    print(f"{peak_ram_mb:.2f},{end_time - start_time:.4f}")