#include <stdio.h>
#include <stdlib.h>
#include <sys/time.h>
#include <sys/mman.h>
#include <pthread.h>
#include <math.h>
#include <unistd.h>

int* Array;
int* Arr2;
int* Arr3;
int* tmpA;
int subarr ;

long size;
int offset;

int num_threads;

pthread_t * tids;
pthread_mutex_t * mutexes;
void print(int arr[], int size);

void *sort_pthread(void *param);
void sort1(int arr[], int size);

void merge( int left, int mid, int right);
void sort( int left, int right);


int main(int argc, char *argv[]) {
     size = 30000000;
     num_threads = 2;

	Array = (int *)malloc(size * sizeof(int));
	Arr2 = (int *)malloc(size * sizeof(int));
	Arr3 = (int *)malloc(size * sizeof(int));



	
	if(argc<=1 || argc>3){
		printf("please enter the size of the array follow by the number of threads\n");
		return 0;
	}
	size = atoi(argv[1]);
	 num_threads = atoi(argv[2]);
	

	subarr = size/num_threads;

	struct timeval stop, start, elapse;

	offset = size % num_threads;


	int i;
	for(i=0; i < size; i++){
		Array[i] = rand() % 100;
	}

//to compare
	i = 0;
	for(i=0;i<size;i++){
		Arr3[i] = Array[i];

	}











	   tids = malloc(num_threads *sizeof(pthread_t));
	   mutexes = malloc(num_threads *sizeof(pthread_mutex_t));
	   i = 0;
	   for(i=0;i<num_threads;i++){
		   pthread_mutex_init(&mutexes[i],NULL);
	   }



    int tmp[num_threads];


    gettimeofday(&start, NULL);

    for (i = 0; i < num_threads; i++) {
        tmp[i] = subarr * i;
        pthread_create(&tids[i], NULL, sort_pthread, &tmp[i]);
	}


    for (i = 0; i < num_threads; i++)
        pthread_join(tids[i], NULL);

    for (i = 1; i <= num_threads-1; i++) {
		int middle = i * subarr;
		merge(0,middle,middle+subarr-1);

	}



	merge(0,(num_threads -1)*subarr,size-1);
	gettimeofday(&stop, NULL);




/*sort1(Arr3,size);
	for (i = 0; i < size; i++)
	{
		if (Arr3[i] != Array[i])
		{
			//printf("%d  error\n", i);
		}
	}


*/



			   printf("Parellel time: %lu :secs %lu :ms\n",stop.tv_sec - start.tv_sec, (long) (stop.tv_usec - start.tv_usec)/1000);

				print(Array,size);


    return 0;
}


void sort1(int arr[], int size)
{
   int i, j, tmp;
   for (i = 0; i < size-1; i++)


       for (j = 0; j < size-i-1; j++){
    	   if (arr[j] > arr[j+1]){
		tmp = arr[j];
		arr[j] = arr[j+1];
		arr[j+1] = tmp;

    	   }
       }

}

void *sort_pthread(void *param) {
    int *left = param;
    int right =  *left + subarr - 1;
    sort(*left, right);

}


void sort(int left,int right) {
    int mid;
    if (right > left) {

	mid = left + (right-left)/2;

        sort( left, mid);
        sort( mid + 1, right);
	/*if(right == size - offset){
	right =size;
	}*/
        merge(left,mid+1,right);
    }
}


void merge( int left, int mid, int right) {
    int i, left_end, count, tmp;
    left_end = mid - 1;
    tmp = left;
    count = right - left + 1;



    while ((left <= left_end) && (mid <= right)) {
        if (Array[left] <= Array[mid]) {
            Arr2[tmp] = Array[left];
			tmp++;
            left++;
        } else {
            Arr2[tmp] = Array[mid];
			tmp++;
            mid++;
        }
    }

//leftovers


    while (left <= left_end) {
        Arr2[tmp] = Array[left];
        tmp++;
        left++;
    }


    while (mid <= right) {
        Arr2[tmp] = Array[mid];
        tmp++;
        mid++;
    }

    int w = right - count+1;

    for (i = 0; i < count; i++) {
        Array[w] = Arr2[w];
        w++;
    }
}
void print( int arr[],int size ){
    int i;
    for (i=0; i < size; i++){
    	printf("%d ", Array[i]);
    }
    	printf("\n");

}

