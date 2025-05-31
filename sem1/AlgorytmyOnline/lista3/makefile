# Makefile

CXX = g++
CXXFLAGS = -std=c++17 -O2 -Wall
OBJ = main.o distributions.o algorithms.o
TARGET = bin_packing

all: $(TARGET)

$(TARGET): $(OBJ)
	$(CXX) $(CXXFLAGS) -o $@ $^

main.o: main.cpp algorithms.hpp distributions.hpp
	$(CXX) $(CXXFLAGS) -c main.cpp

distributions.o: distributions.cpp distributions.hpp
	$(CXX) $(CXXFLAGS) -c distributions.cpp

algorithms.o: algorithms.cpp algorithms.hpp
	$(CXX) $(CXXFLAGS) -c algorithms.cpp

clean:
	rm -f *.o $(TARGET) output.csv
