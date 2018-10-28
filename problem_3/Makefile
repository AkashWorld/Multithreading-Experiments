CC = g++
CFLAGS = -c -Wall -g
LFLAGS = -lm
DFLAGS = -lcrypto -pthread

all: brute_force

brute_force: brute_force.o
	$(CC) $(DFLAGS) brute_force.o -o brute_forcer $(LFLAGS)

brute_force.o: brute_force.cpp brute_force.h
	$(CC) $(CFLAGS) brute_force.cpp

clean:
	rm *.o brute_forcer
