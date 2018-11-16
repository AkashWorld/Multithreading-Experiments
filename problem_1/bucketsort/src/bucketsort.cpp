#include <cstdlib>
#include <iostream>
#include <vector>
#include <ctime>
#include <cassert>
#include <pthread.h>
#define N 100
#define N_BUCKETS 8
#define THRESHOLD 10
#define N_THREADS 4
using namespace std;

struct thread_task
{
    vector<int>& vec;
    int min;
    int max;
    vector<int>& result;
};

void print_vec(const vector<int>& vec){
    for (auto x: vec){
        cout << ' ' << x;
    }
    cout << '\n';
}

vector<int> insertion_sort(const vector<int> &unsorted){
    vector<int> sorted;
    bool flag;
    for (int n : unsorted){
        flag = false;
        for (int i = 0; i < sorted.size(); i++){
            if (n < sorted[i]){
                sorted.insert(sorted.begin() + i, n);
                flag = true;
                break;
            }
        }
        if (!flag){
            sorted.push_back(n);
        }
    }  
    return sorted;
}

vector<int> bucketsort(vector<int> &unsorted, int min, int max) {
    if (unsorted.size() < THRESHOLD){
        return insertion_sort(unsorted);
    }

    vector<int> sorted;
    vector<int> buckets[N_BUCKETS];
    int bucket_size = (max - min) / N_BUCKETS;

    int this_max;
    for (int n : unsorted){
        this_max = min + bucket_size;
        for (int i=0;i<N_BUCKETS;i++){
            if (n < this_max){
                buckets[i].push_back(n);
                break;
            }
            this_max += bucket_size;
        }
    }
    int new_min = min;
    int new_max = min + bucket_size;
    vector<int> sorted_bucket;
    for (int i=0; i<N_BUCKETS-1; i++) {
        sorted_bucket = bucketsort(buckets[i], new_min, new_max); 
        sorted.insert(sorted.end(), sorted_bucket.begin(), sorted_bucket.end());
        new_min = new_max;
        new_max += bucket_size;
    }
    sorted_bucket = bucketsort(buckets[N_BUCKETS-1], new_min, new_max);
    sorted.insert(sorted.end(), sorted_bucket.begin(), sorted_bucket.end());
    return sorted;
}

void *bucketsort_pthread(void *task){
    thread_task* my_task = (thread_task*)task;
    task->result = bucketsort(my_task->vec, my_task->min, my_task->max);
    pthread_exit(NULL);
}

vector<int> bucketsort_threaded(vector<int> &unsorted, int min, int max) {
    vector<int> buckets[N_THREADS];
    int bucket_size = (max - min) / N_THREADS;
    for (int n : unsorted){
        int this_max = min + bucket_size;
        for (int i=0; i<N_THREADS; i++){
            if (n < this_max){
                buckets[i].push_back(n);
                break;
            }
            this_max += bucket_size;
        }
    }

    int new_min = min;
    int new_max = min + bucket_size;
    vector<int> sorted_buckets[N_THREADS];
    pthread_t threads[N_THREADS];
    for (int i=0; i<N_THREADS-1; i++) {
        thread_task *task = (thread_task*)malloc(sizeof(thread_task));
        task->min = new_min;
        task->max = new_max;
        task->vec = buckets[i];
        task->result = vector<int> &sorted_bucket; 
        pthread_create(&threads[i], NULL, bucketsort_pthread, (void*)task);
        new_min = new_max;
        new_max += bucket_size;
    }
    thread_task *task = (thread_task*)malloc(sizeof(thread_task));
    task->min = new_min;
    task->max = max;
    task->vec = buckets[N_THREADS-1];
    task->result = vector<int> &sorted_bucket; 
    pthread_create(&threads[N_THREADS-1], NULL, bucketsort_pthread, (void*)task);
    for (int i=0;i<N_THREADS;i++){
        pthread_join(threads[i], NULL);
    }
    vector<int> sorted;
    for (auto sorted_bucket: sorted_buckets){
        sorted.insert(sorted.end(), sorted_bucket.begin(), sorted_bucket.end());
    }
    return sorted;
}

int main(){
    int min = 0;
    int max = 1000;
    srand(time(nullptr));
    vector<int> unsorted;
    for (int i = 0; i < N; i++){
        unsorted.push_back(min + (rand() % (max - min)));
    }
    printf("unsorted:\n");
    print_vec(unsorted);
    printf("sequential sorted:\n");
    print_vec(bucketsort(unsorted, min, max));
    printf("parell sorted:\n");
    print_vec(bucketsort_threaded(unsorted, min, max));
}

