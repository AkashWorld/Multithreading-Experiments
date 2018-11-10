CC = g++
CFLAGS = -Wall -g 
DFLAGS = -lcrypto -pthread

all: brute_force

brute_force: brute_force.cpp
	$(CC) $(CFLAGS) $(DFLAGS) brute_force.cpp -o brute_forcer

clean:
	rm *.o brute_forcer
