#!/bin/bash

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

PYTHON_PATH="/home/bamichal/miniforge3/envs/tda_stable/bin/python"
CPP_SOURCE_PATH="../RipsersComparisionAndTorus/torusGen.cpp"

echo -e "${BLUE}=== Starting Automated TDA Benchmarks ===${NC}"

if [ ! -f "$CPP_SOURCE_PATH" ]; then
    echo -e "${RED}Error: torus.cpp not found at $CPP_SOURCE_PATH.${NC}"
    exit 1
fi

echo -e "${GREEN}[1/5] Compiling C++ generator...${NC}"
g++ -O3 "$CPP_SOURCE_PATH" -o torus
if [ $? -ne 0 ]; then
    echo -e "${RED}Compilation failed.${NC}"
    exit 1
fi

if [ ! -f "$PYTHON_PATH" ]; then
    echo -e "${RED}Error: Python interpreter not found at $PYTHON_PATH${NC}"
    exit 1
fi

echo -e "${GREEN}[2/5] Running Cohomology Benchmarks (Ripser & GUDHI)...${NC}"
"$PYTHON_PATH" benchmark_cohom.py

echo -e "${GREEN}[3/5] Running PHAT Generation Benchmarks...${NC}"
"$PYTHON_PATH" benchmark_phat_gen.py

echo -e "${GREEN}[4/5] Running PHAT Standard Reductions Benchmarks...${NC}"
"$PYTHON_PATH" benchmark_phat_standard.py

echo -e "${GREEN}[5/5] Generating Combined Plots...${NC}"
"$PYTHON_PATH" plot_results.py

echo -e "${YELLOW}=== Benchmarks Completed! Check the 'plots' folder for .png files ===${NC}"