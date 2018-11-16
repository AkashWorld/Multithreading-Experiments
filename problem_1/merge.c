#include <stdio.h>
#include <stdlib.h>
#include <sys/time.h>
#include <sys/mman.h>
#include <pthread.h>
#include <math.h>
#include <unistd.h>
#include <string.h>

int *Array;
int *Arr2;
int *Arr3;
int *ans;
int subarr;
int *Arr;
long size;
int offset;
int num_threads;
pthread_t *tids;
pthread_mutex_t *mutexes;
void print(int arr[], int size);
void *sort_pthread(void *param);
void sort1(int arr[], int size);
void merge(int left, int mid, int right);
void sort(int left, int right);
void merge2(int left, int mid, int right);
void sort2(int left, int right);

int main(int argc, char *argv[])
{
	size = 400000150;
	num_threads = 2;
	int y = 1;
	int r;
	Array = (int *)malloc((num_threads + size) * sizeof(int));
	Arr2 = (int *)malloc(size * sizeof(int));
	Arr3 = (int *)malloc(size * sizeof(int));
	Arr = (int *)malloc(size * sizeof(int));
	ans = (int *)malloc(size * sizeof(int));

	// ./bubble 2000 4 -s
	if (argc <= 1 || argc > 4)
	{
		printf("Argument format: size of array, number of threads, optional -s flag for sequential sort\n Example: ./mergesort 8000 4\n");
		return 0;
	}
	size = atoi(argv[1]);
	num_threads = atoi(argv[2]);

	struct timeval stop, start;

	r = size % num_threads;
	offset = num_threads - r;
	//printf("%d %d\n",r, offset);

	int i;
	for (i = 0; i < size; i++)
	{
		Array[i] = rand() % 100;
	}

	i = 0;
	for (i = 0; i < size; i++)
	{
		Arr[i] = Array[i];
	}

	//to compare
	/*
	i = 0;
	for(i=0;i<size;i++){
		Arr3[i] = Array[i];

	}


*/
	i = 0;
	int x = size;
	for (i = 0; i < offset; i++)
	{
		Array[x + i] = 101;
	}

	if (argc == 4)
	{
		//print(Arr, size); //see unsorted array
		if (!strcmp(argv[3], "-s"))
		{
			gettimeofday(&start, NULL);
			sort2(0, size - 1);
			gettimeofday(&stop, NULL);

			//print(Arr, size); //see seq sorted array

			printf("Sequential time: %lu secs, %lu ms\n", stop.tv_sec - start.tv_sec, (long)(stop.tv_usec - start.tv_usec) / 1000);
		}
	}
	free(Arr);
	x = size;
	size = size + offset;
	subarr = size / num_threads;

	tids = malloc(num_threads * sizeof(pthread_t));
	mutexes = malloc(num_threads * sizeof(pthread_mutex_t));
	i = 0;
	for (i = 0; i < num_threads; i++)
	{
		pthread_mutex_init(&mutexes[i], NULL);
	}

	int tmp[num_threads];

	gettimeofday(&start, NULL);

	for (i = 0; i < num_threads; i++)
	{
		tmp[i] = subarr * i;
		pthread_create(&tids[i], NULL, sort_pthread, &tmp[i]);
	}

	for (i = 0; i < num_threads; i++)
	{
		pthread_join(tids[i], NULL);
	}

	for (i = 1; i <= num_threads - 1; i++)
	{
		int middle = i * subarr;
		merge(0, middle, middle + subarr - 1);
	}

	merge(0, (num_threads - 1) * subarr, size - 1);
	gettimeofday(&stop, NULL);

	printf("Parallel time: %lu secs, %lu ms\n", stop.tv_sec - start.tv_sec, (long)(stop.tv_usec - start.tv_usec) / 1000);
	i = 0;
	for (i = 0; i < x; i++)
	{
		ans[i] = Array[i];
	}

	//	print(ans,x); // to see parallel sorted arr

	return 0;
}

//initially used bubblesort to compare, useless class
void sort1(int arr[], int size)
{
	int i, j, tmp;
	for (i = 0; i < size - 1; i++)

		for (j = 0; j < size - i - 1; j++)
		{
			if (arr[j] > arr[j + 1])
			{
				tmp = arr[j];
				arr[j] = arr[j + 1];
				arr[j + 1] = tmp;
			}
		}
}
// starts recursive sort
void *sort_pthread(void *param)
{
	int *left = param;
	int right = *left + subarr - 1;
	sort(*left, right);
}

// split array into chunks
void sort(int left, int right)
{
	int mid;
	if (right > left)
	{

		mid = left + (right - left) / 2;

		sort(left, mid);
		sort(mid + 1, right);
		/*if(right == size - offset){
	right =size;
	}*/
		merge(left, mid + 1, right);
	}
}

//merge together
void merge(int left, int mid, int right)
{
	int i, left_end, count, tmp;
	left_end = mid - 1;
	tmp = left;
	count = right - left + 1;

	while ((left <= left_end) && (mid <= right))
	{
		if (Array[left] <= Array[mid])
		{
			Arr2[tmp] = Array[left];
			tmp++;
			left++;
			continue;
		}
		Arr2[tmp] = Array[mid];
		tmp++;
		mid++;
	}

	//leftovers

	while (left <= left_end)
	{
		Arr2[tmp] = Array[left];
		tmp++;
		left++;
	}

	while (mid <= right)
	{
		Arr2[tmp] = Array[mid];
		tmp++;
		mid++;
	}

	int w = right - count + 1;

	for (i = 0; i < count; i++)
	{
		Array[w] = Arr2[w];
		w++;
	}
}

void print(int arr[], int x)
{
	int i;
	for (i = 0; i < x; i++)
	{
		printf("%d ", arr[i]);
	}
	printf("\n");
}

//everything below is reimplimented fro seq ver
void merge2(int left, int mid, int right)
{
	int i = 0;
	int j = 0;
	int loc = left;
	int start = mid - left + 1;
	int end = right - mid;

	int *tmparr = (int *)malloc((start + 1) * sizeof(int));
	int *tmparr2 = (int *)malloc((end + 1) * sizeof(int));
	//int tmparr[start], tmparr2[end];

	for (i = 0; i < start; i++)
	{
		tmparr[i] = Arr[left + i];
	}

	for (j = 0; j < end; j++)
	{
		tmparr2[j] = Arr[mid + 1 + j];
	}

	i = 0;
	j = 0;

	while (i < start && j < end)
	{
		if (tmparr[i] <= tmparr2[j])
		{
			Arr[loc] = tmparr[i];
			i++;
			loc++;
			continue;
		}
		Arr[loc] = tmparr2[j];
		j++;

		loc++;
	}

	while (i < start)
	{
		Arr[loc] = tmparr[i];
		i++;
		loc++;
	}

	while (j < end)
	{
		Arr[loc] = tmparr2[j];
		j++;
		loc++;
	}
}

void sort2(int left, int right)
{
	if (left < right)
	{
		int mid = left + (right - left) / 2;
		sort2(left, mid);
		sort2(mid + 1, right);

		merge2(left, mid, right);
	}
}
