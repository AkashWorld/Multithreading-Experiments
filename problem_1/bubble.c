#include <stdio.h>
#include <stdlib.h>
#include <time.h>

void print(int arr[], int size);
void sort(int arr[], int size) ;


int main(void) {

	srand(time(NULL));

	int Array[25];

	int size = sizeof(Array)/sizeof(Array[0]);

Array[0] = rand() % 20;



	int i;
	for(i=0; i < size; i++){
		Array[i] = rand() % 100;
	}


	print(Array,size);
	sort(Array,size);
	printf("\n");
	print(Array,size);
	printf("\n");
	return 0;
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

