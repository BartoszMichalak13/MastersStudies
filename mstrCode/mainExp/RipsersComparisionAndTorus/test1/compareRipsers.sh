#!/bin/bash

DIM=2
NUM_POINTS=1000
POINTS_FILE="points.txt"
CPP_LOG="ripser_cpp.txt"
CPP_STATS="ripser_cpp_stats.txt"

echo "=== STEP 1: Compile and generate points ==="
g++ -O3 ./../torusGen.cpp -o ./torusGen.out
./torusGen.out $NUM_POINTS 3 > $POINTS_FILE
echo "Generated file: $POINTS_FILE"

echo ""
echo "=== STEP 1.5: Ensure C++ Ripser is built ==="
if [ ! -f "./../../ripser/ripser" ]; then
    echo "Ripser executable not found! Attempting to build it..."

    if [ ! -d "./../../ripser" ]; then
        echo "ERROR: The 'ripser' directory does not exist."
        echo "Please run: git clone https://github.com/Ripser/ripser.git"
        exit 1
    fi

    cd ../../ripser
    make
    cd ..
    echo "Build complete."
else
    echo "Ripser executable found. Skipping build."
fi

echo ""
echo "=== STEP 2: Run original Ripser (C++) ==="

chmod +x ./../../ripser/ripser

/usr/bin/time -v ./../../ripser/ripser --format point-cloud --dim $DIM $POINTS_FILE > $CPP_LOG 2> $CPP_STATS

echo "Ripser C++ finished."

echo ""
echo "=== STEP 3: Run analysis in Python ==="
/home/bamichal/miniforge3/envs/tda_stable/bin/python ./compareRipsers.py $POINTS_FILE $CPP_LOG $CPP_STATS $DIM