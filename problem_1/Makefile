all: bubblesort mergesort

bubblesort: bubble.c
	gcc -pthread -o bubblesort bubble.c -lm

mergesort: merge.c
	gcc  -pthread -o mergesort merge.c -lm


clean:
	rm bubblesort mergesort 
