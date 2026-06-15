#!/bin/bash

DIM=2
RESULTS="benchmark_results.csv"
THRESH=2.0

g++ -O3 ./../torusGen.cpp -o ./torusGen.out

echo "N,CPP_RAM,CPP_TIME,PY_NO_THR_RAM,PY_NO_THR_TIME,PY_THR_RAM,PY_THR_TIME,SUB_RAM,SUB_TIME,GIOTTO_RAM,GIOTTO_TIME" > $RESULTS

POINTS_ARRAY=(50 100 150 200 250 300 350 400 450 500)

echo "=== STARTING BENCHMARK LOOP ==="
for N in "${POINTS_ARRAY[@]}"; do
    echo "Testing for N = $N..."
    ./torusGen.out $N 3 > points.txt

    /usr/bin/time -v ./../../ripser/ripser --format point-cloud --dim $DIM --threshold $THRESH points.txt > /dev/null 2> cpp_stats.txt
    
    CPP_RAM=$(awk '/Maximum resident set size/ {printf "%.2f", $6/1024}' cpp_stats.txt)
    CPP_TIME=$(awk '/User time/ {print $4}' cpp_stats.txt)

    PY_NO_OUT=$(/home/bamichal/miniforge3/envs/tda_stable/bin/python measure_py.py points.txt $DIM -1)
    PY_NO_RAM=$(echo $PY_NO_OUT | cut -d',' -f1)
    PY_NO_TIME=$(echo $PY_NO_OUT | cut -d',' -f2)

    PY_THR_OUT=$(/home/bamichal/miniforge3/envs/tda_stable/bin/python measure_py.py points.txt $DIM $THRESH)
    PY_THR_RAM=$(echo $PY_THR_OUT | cut -d',' -f1)
    PY_THR_TIME=$(echo $PY_THR_OUT | cut -d',' -f2)

    SUB_OUT=$(/home/bamichal/miniforge3/envs/tda_stable/bin/python measure_subprocess.py points.txt $DIM ./../../ripser/ripser $THRESH)
    SUB_RAM=$(echo $SUB_OUT | cut -d',' -f1)
    SUB_TIME=$(echo $SUB_OUT | cut -d',' -f2)

    GIOTTO_OUT=$(/home/bamichal/miniforge3/envs/tda_stable/bin/python measure_giotto.py points.txt $DIM $THRESH)
    GIOTTO_RAM=$(echo $GIOTTO_OUT | cut -d',' -f1)
    GIOTTO_TIME=$(echo $GIOTTO_OUT | cut -d',' -f2)

    echo "$N,$CPP_RAM,$CPP_TIME,$PY_NO_RAM,$PY_NO_TIME,$PY_THR_RAM,$PY_THR_TIME,$SUB_RAM,$SUB_TIME,$GIOTTO_RAM,$GIOTTO_TIME" >> $RESULTS
done

echo ""
echo "=== BENCHMARK COMPLETE ==="
/home/bamichal/miniforge3/envs/tda_stable/bin/python plot_optimized.py $RESULTS