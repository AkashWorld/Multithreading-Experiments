#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <semaphore.h>
#include <pthread.h>
#include <assert.h>

void print(int arr[], int size);
void sort(int arr[], int size);
void* sort_pthread(void *param);

int num_threads;

pthread_t * tids;
pthread_mutex_t * mutexes;

int* Array;
int* Arr2;
long size = 10000000;

int main(void) {


clock_t start, stop;



Array = (int*) malloc(size*sizeof(int));


srand(time(NULL));

	int i;
	for(i=0; i < size; i++){
		Array[i] = rand() % 100;
	}


   double t;

  assert((start=clock())!=-1);
  	sort(Array,size);
   stop = clock();
   t = (double) (stop-start)/(CLOCKS_PER_SEC);



   printf("Sequential Time: %g\n", t/100);

	//print(Array,size);

	//printf("\n");
   //thread imple



Arr2 = (int*) malloc(size*sizeof(int));

	i = 0;
	for(i=0;i<size;i++){
		Arr2[i] = Array[i];

	}


	num_threads = 4;

	   tids = malloc(num_threads *sizeof(pthread_t));
	   mutexes = malloc(num_threads *sizeof(pthread_mutex_t));
	   i = 0;
	   for(i=0;i<num_threads;i++){
		   pthread_mutex_init(&mutexes[i],NULL);
	   }





	       for(i=0;i<num_threads;i++){

	           pthread_create(&tids[i],NULL,sort_pthread,(void*)0);
	       }
	       for(i=0; i<num_threads;i++){
	    	   pthread_join(tids[i],NULL);
	       }

	  // 	print(Arr2,size);

	 //  	printf("\n");

	       for( i = 0; i < size; i++){
	    	   if(Arr2[i] != Array[i]){
	    		   printf("%d  error\n",i);
	    	   }

	       }

return 0;
}


void* sort_pthread(void *param) {
	 int i=0,k=0, tmp=0, start=0, end=0;


	  int  per_thread = size/num_threads;

	  for(i = 0; i<per_thread; i++){
		  for(k=0;k<num_threads;k++){
			  start = per_thread * k;
			  end = start + per_thread;

			  if(k == (num_threads-1) ){
				  end = size;
			  }
			  if(k==0) pthread_mutex_lock(&mutexes[0]);

	            for(i=start;i<end-1;i++){
	                if(Arr2[i]>Arr2[i+1]){
	                    tmp=Arr2[i];
	                    Arr2[i]=Arr2[i+1];
	                    Arr2[i+1]=tmp;
	                }
	            }

	            if(k==num_threads-1){
	                pthread_mutex_unlock(&mutexes[k]);
	                break;
	            }
/*
	            pthread_mutex_lock(&mutexes[k+1]);
                if(Arr2[i]>Arr2[i+1]){
                    tmp=Arr2[i];
                    Arr2[i]=Arr2[i+1];
                    Arr2[i+1]=tmp;
                }

                */
	            pthread_mutex_unlock(&mutexes[k]);

		  }
	  }


	return NULL;
}


void sort(int arr[], int size)
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


void print( int arr[],int size ){
    int i;
    for (i=0; i < size; i++){
    	printf("%d ", arr[i]);
    }

}

