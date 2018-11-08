#include <stdio.h>
#include <stdlib.h>
#include <time.h>

void print(int arr[], int size);
void sort(int arr[], int left, int right);
void merge(int arr[], int left, int mid, int right);

int main(void) {

     clock_t start, end;
     double elapsed_t;

	srand(time(NULL));

	int Array[25];

	int size = sizeof(Array)/sizeof(Array[0]);

Array[0] = rand() % 20;



	int i;
	for(i=0; i < size; i++){
		Array[i] = rand() % 100;
	}


	print(Array,size);

start = clock();
	sort(Array,0,size-1);
end= clock();
 elapsed_t = ((double) (end - start)) / CLOCKS_PER_SEC;
	printf("\n");
	print(Array,size);
	printf("\n");
	return 0;
}



void sort(int arr[], int left, int right)
{
    if (right > left)
    {

        int mid = (left+right)/2;

    	//int mid2 = left+(right-left)/2;
    	//printf("%d  %d\n",mid,mid2);
        sort(arr, left, mid);
        sort(arr, mid + 1, right);

        merge(arr, left, mid, right);
    }

}

void merge(int arr[], int left, int mid, int right){


	int larr = mid - left - 1;
	int rarr = right - mid;

	int L[larr], R[rarr];

	int i = 0;
	for (i = 0; i < larr; i++){
		 L[i] = arr[left + i];
	}
	for (i = 0; i < rarr; i++){
		 R[i] = arr[mid + i + 1];
	}

	i=0;
	int x = 0, y = left;
    while (i < larr && x < rarr)
    {
        if (L[i] <= R[x])
        {
            arr[y] = L[i];
            i++;
        }
        else
        {
            arr[y] = R[x];
            x++;
        }
        y++;
    }



}



void print( int arr[],int size ){
    int i;
    for (i=0; i < size; i++){
    	printf("%d ", arr[i]);
    }

}

