all: bubblesort mergesort

bubblesort: bubble.c
	$(CC) -o bubblesort bubble.c -lm -pthread 

mergesort: merge.c
	$(CC) -o mergesort merge.c -lm -pthread 
.
bucketsort: bucketsort.cpp
	$(CXX) -std=c++11 bucketsort.cpp -o bucketsort -pthread
clean:
	rm bubblesort mergesort bucketsort
