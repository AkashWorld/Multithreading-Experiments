#include <cstdlib>
#include <iostream>
#include <vector>
#include <ctime>
#include <time.h>
#include<sys/time.h> 
#include <cassert>
#include <pthread.h> 
#define N_BUCKETS 4
#define THRESHOLD 10
#define N_THREADS 8
using namespace std;

struct thread_task
{
    vector<int> *vec;
    int min;
    int max;
    int start;
    int *sorted;
};

void print_vec(const vector<int> &vec)
{
    for (auto x : vec)
        cout << ' ' << x;
    cout << '\n';
}

vector<int> insertion_sort(const vector<int> &unsorted)
{
    vector<int> sorted;
    bool flag;
    for (int n : unsorted)
    {
        flag = false;
        for (int i = 0; i < sorted.size(); i++)
        {
            if (n < sorted[i])
            {
                sorted.insert(sorted.begin() + i, n);
                flag = true;
                break;
            }
        }
        if (!flag)
            sorted.push_back(n);
    }
    return sorted;
}

vector<int> bucketsort(vector<int> &unsorted, int min, int max)
{
    if (unsorted.size() < THRESHOLD)
        return insertion_sort(unsorted);

    vector<int> buckets[N_BUCKETS];
    int bucket_size = (max - min) / N_BUCKETS;
    int this_max;
    for (int n : unsorted)
    {
        this_max = min + bucket_size;
        for (int i = 0; i < N_BUCKETS; i++)
        {
            if (n < this_max)
            {
                buckets[i].push_back(n);
                break;
            }
            this_max += bucket_size;
        }
    }

    int new_min = min;
    int new_max = min + bucket_size;
    vector<int> sorted_bucket;
    vector<int> sorted;
    for (int i = 0; i < N_BUCKETS - 1; i++)
    {
        sorted_bucket = bucketsort(buckets[i], new_min, new_max);
        sorted.insert(sorted.end(), sorted_bucket.begin(), sorted_bucket.end());
        new_min = new_max;
        new_max += bucket_size;
    }
    sorted_bucket = bucketsort(buckets[N_BUCKETS - 1], new_min, new_max);
    sorted.insert(sorted.end(), sorted_bucket.begin(), sorted_bucket.end());
    return sorted;
}

void *bucketsort_pthread(void *task)
{
    thread_task *my_task = (thread_task *)task;
    vector<int> sorted_bucket = bucketsort(*(my_task->vec), my_task->min, my_task->max);
    int i = my_task->start;
    for (int n : sorted_bucket)
    {
        my_task->sorted[i] = n;
        i++;
    }
    pthread_exit(NULL);
}

vector<int> bucketsort_threaded(vector<int> &unsorted, int min, int max)
{
    vector<int> buckets[N_THREADS];
    int bucket_size = (max - min) / N_THREADS;
    for (int n : unsorted)
    {
        int this_max = min + bucket_size;
        for (int i = 0; i < N_THREADS; i++)
        {
            if (n < this_max)
            {
                buckets[i].push_back(n);
                break;
            }
            this_max += bucket_size;
        }
    }
    int new_min = min;
    int new_max = min + bucket_size;
    int* sorted = (int*)malloc(unsorted.size()*sizeof(int));
    pthread_t threads[N_THREADS];
    int start = 0;
    thread_task tasks[N_THREADS];
    for (int i = 0; i < N_THREADS - 1; i++)
    {
        tasks[i].min = new_min;
        tasks[i].max = new_max;
        tasks[i].start = start;
        tasks[i].sorted = sorted;
        tasks[i].vec = &(buckets[i]);

        pthread_create(&threads[i], NULL, bucketsort_pthread, (void *)&tasks[i]);

        new_min = new_max;
        new_max += bucket_size;
        start += buckets[i].size() ;
    }
    tasks[N_THREADS - 1].min = new_min;
    tasks[N_THREADS - 1].max = max;
    tasks[N_THREADS - 1].vec = &(buckets[N_THREADS - 1]);
    tasks[N_THREADS - 1].start = start;
    tasks[N_THREADS - 1].sorted = sorted;
    pthread_create(&threads[N_THREADS - 1], NULL, bucketsort_pthread, (void *)&tasks[N_THREADS - 1]);
    for (int i = 0; i < N_THREADS; i++)
        pthread_join(threads[i], NULL);
    vector<int> sorted_vec;
    for (int i = 0; i < unsorted.size(); i++)
        sorted_vec.push_back(sorted[i]);
    free(sorted);
    return sorted_vec;
}

int main(int argc, char *argv[])
{
    
    int min = 0;
    int max = 100;
    int n = 100;
    srand(time(nullptr));
    vector<int> unsorted;
    for (int i = 0; i < n; i++)
        unsorted.push_back(min + (rand() % (max - min)));

    struct timeval stop, start;

    printf("sorting sequentially\n");
    gettimeofday(&start, NULL);
    vector<int> seq_sorted = bucketsort(unsorted, min, max);
    print_vec(seq_sorted);
    gettimeofday(&stop, NULL);
    printf("Sequential time: %lu secs, %lu ms\n", stop.tv_sec - start.tv_sec, (long)(stop.tv_usec - start.tv_usec) / 1000);

    printf("sorting parallel\n");
    gettimeofday(&start, NULL);
    vector<int> par_sorted = bucketsort_threaded(unsorted, min, max);
    gettimeofday(&stop, NULL);
    print_vec(par_sorted);
    printf("Parallel time: %lu secs, %lu ms\n", stop.tv_sec - start.tv_sec, (long)(stop.tv_usec - start.tv_usec) / 1000);
}
