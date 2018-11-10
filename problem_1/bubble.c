#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <semaphore.h>
#include <pthread.h>
#include <assert.h>
#include <sys/time.h>
#include <math.h>

void print(int arr[], int size);
void sort(int arr[], int size);
void* sort_pthread(void *param);

int num_threads;

pthread_t * tids;
pthread_mutex_t * mutexes;

int* Array;
int* Arr2;
long size;

int main(int argc, char* argv[]) {

size = 23;
num_threads = 4;

/*
if(argc<=1 || argc>3){
	printf("please enter the size of the array follow by the number of threads\n");
	return 0;

}

int size = atoi(argv[1]);
int num_threads = atoi(argv[2]);
*/

struct timeval stop, start;


Array = (int*) malloc(size*sizeof(int));


srand(time(NULL));

	int i;
	for(i=0; i < size; i++){
		Array[i] = rand() % 100;
	}
Arr2 = (int*) malloc(size*sizeof(int));

	i = 0;
	for(i=0;i<size;i++){
		Arr2[i] = Array[i];

	}

gettimeofday(&start, NULL);
sort(Array,size);
gettimeofday(&stop, NULL);

long x =stop.tv_usec - start.tv_usec;

printf("sequential time: %lu :secs %lu :ms\n",stop.tv_sec - start.tv_sec, (long) x/1000);



	print(Array,size);

	printf("\n");
   //thread imple








	   tids = malloc(num_threads *sizeof(pthread_t));
	   mutexes = malloc(num_threads *sizeof(pthread_mutex_t));
	   i = 0;
	   for(i=0;i<num_threads;i++){
		   pthread_mutex_init(&mutexes[i],NULL);
	   }





gettimeofday(&start, NULL);
	       for(i=0;i<num_threads;i++){

	           pthread_create(&tids[i],NULL,sort_pthread,(void*)0);
	       }
	       for(i=0; i<num_threads;i++){
	    	   pthread_join(tids[i],NULL);
	       }




gettimeofday(&stop, NULL);

x =stop.tv_usec - start.tv_usec;

printf("Parellel time: %lu :secs %lu :ms\n",stop.tv_sec - start.tv_sec, (long) x/1000);

	  	print(Arr2,size);

	   	printf("\n");

	       for( i = 0; i < size; i++){
	    	   if(Arr2[i] != Array[i]){
	    		   printf("%d  error\n",i);
	    	   }

	       }

return 0;
}


void* sort_pthread(void *param) {



	int  per_thread = size/num_threads;
	int start,end;

	int i = 0;
	int j = 0;
	int k = 0;
	int tmp = 0;
	    for(j = 0; j<floor(per_thread); j++){
	        for(k=0;k<num_threads;k++){
	        		if(k==0) pthread_mutex_lock(&mutexes[0]);
	            start = per_thread *k;
	            end = start + per_thread;
	            if(k==num_threads-1){
	            	end = size;
	            }


	            for(i=start;i<end-1;i++){



	                        if(Arr2[i]>Arr2[i+1]){
	                            tmp=Arr2[i];
	                            Arr2[i]=Arr2[i+1];
	                            Arr2[i+1]=tmp;

		//	printf("%d %d---\n",start,end);
	                        }


	            }
	            if(k==num_threads-1){

	            }else{
	                pthread_mutex_lock(&mutexes[k+1]);
	     	//	printf("%d %d-----------last\n",i,k);
	                     if(Arr2[i]>Arr2[i+1]){
	                         tmp=Arr2[i];
	                         Arr2[i]=Arr2[i+1];
	                         Arr2[i+1]=tmp;
	                     }
	                     if(k == 0){
	     	               if(Arr2[i-1]>Arr2[i]){
	                         tmp=Arr2[i-1];
	                         Arr2[i-1]=Arr2[i];
	                         Arr2[i]=tmp;
	     	               }
	                     }
	            }
	 	//printf("lock %d\n",k+1);

		//printf("unlock %d\n",k);
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

