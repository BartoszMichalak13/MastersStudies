#!/bin/bash

DIM=3
NUM_POINTS=1000
THRESH=2.0
POINTS_FILE="points.txt"
CPP_LOG="ripser_cpp.txt"
CPP_STATS="ripser_cpp_stats.txt"

echo "=== STEP 1: Compile and generate points ==="
g++ -O3 ./../torusGen.cpp -o ./torusGen.out
./torusGen.out $NUM_POINTS 3 > $POINTS_FILE
echo "Generated file: $POINTS_FILE"

echo ""
echo "=== STEP 2: Run original Ripser (C++) ==="
chmod +x ./../../ripser/ripser

/usr/bin/time -v ./../../ripser/ripser --format point-cloud --dim $DIM --threshold $THRESH $POINTS_FILE > $CPP_LOG 2> $CPP_STATS

echo "Ripser C++ finished. Results saved to $CPP_LOG."

echo ""
echo "=== STEP 3: Run visual analysis in Python ==="
/home/bamichal/miniforge3/envs/tda_stable/bin/python ./cppRipserPlot.py $CPP_LOG $DIM

echo "All plots generated successfully!"