#!/bin/bash

DIM=2
RESULTS="benchmark_results.csv"

# Compile the torus generator
g++ -O3 ./../torusGen.cpp -o ./torusGen.out

# CSV header
echo "N,CPP_RAM,CPP_TIME,PY_RAM,PY_TIME" > $RESULTS

# Points to test
POINTS_ARRAY=(50 100 150 200 250 300 350 400 450 500)

echo "=== STARTING BENCHMARK LOOP ==="

for N in "${POINTS_ARRAY[@]}"; do
    echo "Testing for N = $N..."
    ./torusGen.out $N > points.txt

    # 1. Run C++ Native Ripser
    /usr/bin/time -v ./../../ripser/ripser --format point-cloud --dim $DIM points.txt > /dev/null 2> cpp_stats.txt
    
    # Get RAM (in MB) and Time (in seconds) straight from the stats file
    CPP_RAM=$(awk '/Maximum resident set size/ {printf "%.2f", $6/1024}' cpp_stats.txt)
    CPP_TIME=$(awk '/User time/ {print $4}' cpp_stats.txt)

    # 2. Run Python Wrapper
    PY_OUT=$(/home/bamichal/miniforge3/envs/tda_stable/bin/python measure_py.py points.txt $DIM)
    
    # Split to RAM and Time
    PY_RAM=$(echo $PY_OUT | cut -d',' -f1)
    PY_TIME=$(echo $PY_OUT | cut -d',' -f2)

    # 3. Write results to CSV
    echo "$N,$CPP_RAM,$CPP_TIME,$PY_RAM,$PY_TIME" >> $RESULTS
done

echo ""
echo "=== BENCHMARK COMPLETE ==="
echo "Generating plots..."
/home/bamichal/miniforge3/envs/tda_stable/bin/python plot_benchmark.py $RESULTS