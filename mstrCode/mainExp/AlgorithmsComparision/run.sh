#!/bin/bash

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# Ustawiamy ścieżkę do interpretera na sztywno, żeby nie było niedomówień
PYTHON_PATH="/home/bamichal/miniforge3/envs/tda_stable/bin/python"
CPP_SOURCE_PATH="../RipsersComparisionAndTorus/torusGen.cpp"

echo -e "${BLUE}=== Starting Automated TDA Benchmarks ===${NC}"

# 1. Compilation check
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

# 2. Run Python loop
echo -e "${GREEN}[2/2] Running Python Benchmark Loop...${NC}"

# Sprawdzamy czy interpreter istnieje
if [ -f "$PYTHON_PATH" ]; then
    "$PYTHON_PATH" benchmark.py
else
    echo -e "${RED}Error: Python interpreter not found at $PYTHON_PATH${NC}"
    exit 1
fi

echo -e "${YELLOW}=== Benchmarks Completed! Check the .png files ===${NC}"


# GREEN='\033[0;32m'
# BLUE='\033[0;34m'
# YELLOW='\033[1;33m'
# RED='\033[0;31m'
# NC='\033[0m'


# CPP_SOURCE_PATH="../RipsersComparisionAndTorus/torusGen.cpp"

# echo -e "${BLUE}=== Starting Automated TDA Benchmarks ===${NC}"

# # 1. Compilation check
# if [ ! -f "$CPP_SOURCE_PATH" ]; then
#     echo -e "${RED}Error: torus.cpp not found at $CPP_SOURCE_PATH.${NC}"
#     exit 1
# fi

# echo -e "${GREEN}[1/2] Compiling C++ generator...${NC}"
# g++ -O3 "$CPP_SOURCE_PATH" -o torus
# if [ $? -ne 0 ]; then
#     echo -e "${RED}Compilation failed.${NC}"
#     exit 1
# fi

# # 2. Run Python loop (using $CONDA_PREFIX to ensure the correct Conda environment is used)
# echo -e "${GREEN}[2/2] Running Python Benchmark Loop...${NC}"
# "$CONDA_PREFIX/bin/python" benchmark.py

# echo -e "${YELLOW}=== Benchmarks Completed! Check the .png files ===${NC}"