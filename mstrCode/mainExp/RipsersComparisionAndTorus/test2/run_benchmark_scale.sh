# #!/bin/bash

# DIM=2
# THRESH=1.0 # Remember to set a good value, e.g., 1.0 or 0.8 for the torus!
# RESULTS="scale_results.csv"

# # Compile the generator (just in case)
# g++ -O3 ./../torusGen.cpp -o ./torusGen.out

# # Clean CSV header - only the 3 optimized versions
# echo "N,CPP_RAM,CPP_TIME,PY_THR_RAM,PY_THR_TIME,SUB_RAM,SUB_TIME" > $RESULTS

# # Much larger points array!
# POINTS_ARRAY=(100 200 300 400 500 600 700 800 900 1000 1100 1200 1300 1400 1500 1600 1700 1800 1900 2000 2100 2200 2300 2400 2500 2600 2700 2800 2900 3000 3100 3200 3300 3400 3500 3600 3700 3800 3900 4000 4100 4200 4300 4400 4500 4600 4700 4800 4900 5000)

# echo "=== STARTING HIGH-SCALE BENCHMARK LOOP ==="

# for N in "${POINTS_ARRAY[@]}"; do
#     echo "Testing for N = $N..."
#     ./torusGen.out $N 3 > points.txt

#     # 1. Native C++ (with threshold)
#     /usr/bin/time -v ./../../ripser/ripser --format point-cloud --dim $DIM --threshold $THRESH points.txt > /dev/null 2> cpp_stats.txt
#     CPP_RAM=$(awk '/Maximum resident set size/ {printf "%.2f", $6/1024}' cpp_stats.txt)
#     CPP_TIME=$(awk '/User time/ {print $4}' cpp_stats.txt)

#     # 2. Python Wrapper WITH threshold
#     PY_THR_OUT=$(/home/bamichal/miniforge3/envs/tda_stable/bin/python measure_py.py points.txt $DIM $THRESH)
#     PY_THR_RAM=$(echo $PY_THR_OUT | cut -d',' -f1)
#     PY_THR_TIME=$(echo $PY_THR_OUT | cut -d',' -f2)

#     # 3. Subprocess WITH threshold
#     SUB_OUT=$(/home/bamichal/miniforge3/envs/tda_stable/bin/python measure_subprocess.py points.txt $DIM ./../../ripser/ripser $THRESH)
#     SUB_RAM=$(echo $SUB_OUT | cut -d',' -f1)
#     SUB_TIME=$(echo $SUB_OUT | cut -d',' -f2)

#     # Write to CSV
#     echo "$N,$CPP_RAM,$CPP_TIME,$PY_THR_RAM,$PY_THR_TIME,$SUB_RAM,$SUB_TIME" >> $RESULTS
# done

# echo ""
# echo "=== HIGH-SCALE BENCHMARK COMPLETE ==="
# echo "Generating scale plots..."
# /home/bamichal/miniforge3/envs/tda_stable/bin/python plot_scale.py $RESULTS


#!/bin/bash

DIM=2
THRESH=2.0 # Remember to set a good value, e.g., 1.0 or 0.8 for the torus!
RESULTS="scale_results.csv"

# Compile the generator (just in case)
g++ -O3 ./../torusGen.cpp -o ./torusGen.out

# Clean CSV header - only the 3 optimized versions
echo "N,CPP_RAM,CPP_TIME,PY_THR_RAM,PY_THR_TIME,SUB_RAM,SUB_TIME,GIOTTO_RAM,GIOTTO_TIME" > $RESULTS

# Much larger points array!
POINTS_ARRAY=(100 200 300 400 500 600 700 800 900 1000 1100 1200 1300 1400 1500)
#  1600 1700 1800 1900 2000)
# 2100 2200 2300 2400 2500 2600 2700 2800 2900 3000 3100 3200 3300 3400 3500 3600 3700 3800 3900 4000 4100 4200 4300 4400 4500 4600 4700 4800 4900 5000)

echo "=== STARTING HIGH-SCALE BENCHMARK LOOP ==="

for N in "${POINTS_ARRAY[@]}"; do
    echo "Testing for N = $N..."
    ./torusGen.out $N 3 > points.txt

    # 1. Native C++
    /usr/bin/time -v ./../../ripser/ripser --format point-cloud --dim $DIM --threshold $THRESH points.txt > /dev/null 2> cpp_stats.txt
    CPP_RAM=$(awk '/Maximum resident set size/ {printf "%.2f", $6/1024}' cpp_stats.txt)
    CPP_TIME=$(awk '/User time/ {print $4}' cpp_stats.txt)

    # 2. Python Wrapper (ripser.py)
    PY_THR_OUT=$(/home/bamichal/miniforge3/envs/tda_stable/bin/python measure_py.py points.txt $DIM $THRESH)
    PY_THR_RAM=$(echo $PY_THR_OUT | cut -d',' -f1)
    PY_THR_TIME=$(echo $PY_THR_OUT | cut -d',' -f2)

    # 3. Subprocess
    SUB_OUT=$(/home/bamichal/miniforge3/envs/tda_stable/bin/python measure_subprocess.py points.txt $DIM ./../../ripser/ripser $THRESH)
    SUB_RAM=$(echo $SUB_OUT | cut -d',' -f1)
    SUB_TIME=$(echo $SUB_OUT | cut -d',' -f2)

    # 4. Giotto-TDA (NOWE)
    GIOTTO_OUT=$(/home/bamichal/miniforge3/envs/tda_stable/bin/python measure_giotto.py points.txt $DIM $THRESH)
    GIOTTO_RAM=$(echo $GIOTTO_OUT | cut -d',' -f1)
    GIOTTO_TIME=$(echo $GIOTTO_OUT | cut -d',' -f2)

    # Write to CSV z nowymi zmiennymi
    echo "$N,$CPP_RAM,$CPP_TIME,$PY_THR_RAM,$PY_THR_TIME,$SUB_RAM,$SUB_TIME,$GIOTTO_RAM,$GIOTTO_TIME" >> $RESULTS
done

echo ""
echo "=== HIGH-SCALE BENCHMARK COMPLETE ==="
echo "Generating scale plots..."
/home/bamichal/miniforge3/envs/tda_stable/bin/python plot_scale.py $RESULTS