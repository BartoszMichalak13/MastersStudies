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

echo -e "${GREEN}[1/2] Compiling C++ generator...${NC}"
g++ -O3 "$CPP_SOURCE_PATH" -o torus
if [ $? -ne 0 ]; then
    echo -e "${RED}Compilation failed.${NC}"
    exit 1
fi

echo -e "${GREEN}[2/2] Running Python Benchmark Loop...${NC}"
if [ -f "$PYTHON_PATH" ]; then
    "$PYTHON_PATH" benchmark.py
else
    echo -e "${RED}Error: Python interpreter not found at $PYTHON_PATH${NC}"
    exit 1
fi
echo -e "${YELLOW}=== Benchmarks Completed! Check the .png files ===${NC}"
