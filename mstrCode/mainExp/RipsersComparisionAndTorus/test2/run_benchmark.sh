#!/bin/bash

DIM=2
RESULTS="benchmark_results.csv"
THRESH=2.0

# Compile the torus generator
g++ -O3 ./../torusGen.cpp -o ./torusGen.out

# CSV header
echo "N,CPP_RAM,CPP_TIME,PY_NO_THR_RAM,PY_NO_THR_TIME,PY_THR_RAM,PY_THR_TIME,SUB_RAM,SUB_TIME,GIOTTO_RAM,GIOTTO_TIME" > $RESULTS

# Points to test
POINTS_ARRAY=(50 100 150 200 250 300 350 400 450 500)

echo "=== STARTING BENCHMARK LOOP ==="

# for N in "${POINTS_ARRAY[@]}"; do
#     echo "Testing for N = $N..."
#     ./torusGen.out $N 3 > points.txt

#     # 1. Run C++ Native Ripser
#     /usr/bin/time -v ./../../ripser/ripser --format point-cloud --dim $DIM points.txt > /dev/null 2> cpp_stats.txt

#     # Get RAM (in MB) and Time (in seconds) straight from the stats file
#     CPP_RAM=$(awk '/Maximum resident set size/ {printf "%.2f", $6/1024}' cpp_stats.txt)
#     CPP_TIME=$(awk '/User time/ {print $4}' cpp_stats.txt)

#     # 2. Run Python Wrapper
#     PY_OUT=$(/home/bamichal/miniforge3/envs/tda_stable/bin/python measure_py.py points.txt $DIM)

#     # Split to RAM and Time
#     PY_RAM=$(echo $PY_OUT | cut -d',' -f1)
#     PY_TIME=$(echo $PY_OUT | cut -d',' -f2)

#     # 3. Run subprocess measurement
#     SUB_OUT=$(/home/bamichal/miniforge3/envs/tda_stable/bin/python measure_subprocess.py points.txt $DIM ./../../ripser/ripser)

#     # Split to RAM and Time
#     SUB_RAM=$(echo $SUB_OUT | cut -d',' -f1)
#     SUB_TIME=$(echo $SUB_OUT | cut -d',' -f2)

#     # 4. Write results to CSV
#     echo "$N,$CPP_RAM,$CPP_TIME,$PY_RAM,$PY_TIME,$SUB_RAM,$SUB_TIME" >> $RESULTS
# done

for N in "${POINTS_ARRAY[@]}"; do
    echo "Testing for N = $N..."
    ./torusGen.out $N 3 > points.txt

    # 1. C++ Native Ripser (z progiem dla czystego porównania)
    /usr/bin/time -v ./../../ripser/ripser --format point-cloud --dim $DIM --threshold $THRESH points.txt > /dev/null 2> cpp_stats.txt
    
    CPP_RAM=$(awk '/Maximum resident set size/ {printf "%.2f", $6/1024}' cpp_stats.txt)
    CPP_TIME=$(awk '/User time/ {print $4}' cpp_stats.txt)

    # 2. Python Wrapper BEZ progu (symulacja wybuchu RAM-u, przekazujemy -1)
    PY_NO_OUT=$(/home/bamichal/miniforge3/envs/tda_stable/bin/python measure_py.py points.txt $DIM -1)
    PY_NO_RAM=$(echo $PY_NO_OUT | cut -d',' -f1)
    PY_NO_TIME=$(echo $PY_NO_OUT | cut -d',' -f2)

    # 3. Python Wrapper Z progiem (optymalne zużycie)
    PY_THR_OUT=$(/home/bamichal/miniforge3/envs/tda_stable/bin/python measure_py.py points.txt $DIM $THRESH)
    PY_THR_RAM=$(echo $PY_THR_OUT | cut -d',' -f1)
    PY_THR_TIME=$(echo $PY_THR_OUT | cut -d',' -f2)

    # 4. Subprocess (z progiem)
    SUB_OUT=$(/home/bamichal/miniforge3/envs/tda_stable/bin/python measure_subprocess.py points.txt $DIM ./../../ripser/ripser $THRESH)
    SUB_RAM=$(echo $SUB_OUT | cut -d',' -f1)
    SUB_TIME=$(echo $SUB_OUT | cut -d',' -f2)

    # 5. Giotto-TDA (NOWE)
    GIOTTO_OUT=$(/home/bamichal/miniforge3/envs/tda_stable/bin/python measure_giotto.py points.txt $DIM $THRESH)
    GIOTTO_RAM=$(echo $GIOTTO_OUT | cut -d',' -f1)
    GIOTTO_TIME=$(echo $GIOTTO_OUT | cut -d',' -f2)

    # # 6. Giotto-TDA (NOWE)
    # GIOTTO_NO_OUT=$(/home/bamichal/miniforge3/envs/tda_stable/bin/python measure_giotto.py points.txt $DIM)
    # GIOTTO_NO_RAM=$(echo $GIOTTO_NO_OUT | cut -d',' -f1)
    # GIOTTO_NO_TIME=$(echo $GIOTTO_NO_OUT | cut -d',' -f2)

    # 5. Write results to CSV
    echo "$N,$CPP_RAM,$CPP_TIME,$PY_NO_RAM,$PY_NO_TIME,$PY_THR_RAM,$PY_THR_TIME,$SUB_RAM,$SUB_TIME,$GIOTTO_RAM,$GIOTTO_TIME" >> $RESULTS
done

echo ""
echo "=== BENCHMARK COMPLETE ==="
# /home/bamichal/miniforge3/envs/tda_stable/bin/python plot_benchmark.py $RESULTS
/home/bamichal/miniforge3/envs/tda_stable/bin/python plot_optimized.py $RESULTS