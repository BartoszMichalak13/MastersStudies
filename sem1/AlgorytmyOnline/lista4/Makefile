CXX = g++
CXXFLAGS = -ggdb -O0 -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror
OBJS = main.o
#graph/torus3d.o graph/hypercube.o algorithms/move_to_min.o algorithms/coin_flip.o \
       distributions/uniform.o distributions/harmonic.o distributions/double_harmonic.o utils/rng.o

all: page_migration

page_migration: $(OBJS)
	$(CXX) $(CXXFLAGS) -o $@ $^

clean:
	rm -f page_migration $(OBJS)
