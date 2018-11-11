bubblesort: bubble.c
	gcc -g -Wall -pthread -o bubble bubble.c -lm

mergesort: merge.c
	gcc -g -Wall -o mergesort merge.c

mergesort_p: mergesort_p.c
	gcc -g -Wall -pthread -o merge_p mergesort_p.c -lm

clean: bubble mergesort
	rm bubble mergesort merge_p
