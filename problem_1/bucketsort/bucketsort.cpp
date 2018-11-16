#include <cstdlib>
#include <iostream>
#include <vector>
#include <ctime>
#include <cassert>
#define N 100
#define N_BUCKETS 8
#define THRESHOLD 10
using namespace std;

void print_vec(const vector<int>& vec){
    for (auto x: vec){
        cout << ' ' << x;
    }
    cout << '\n';
}

vector<int> insertion_sort(vector<int> unsorted){
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

vector<int> bucketsort(vector<int> unsorted, int min, int max) {
    // insertion sort if small enough
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

    // sort buckets and concatenate
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
    printf("sorted:\n");
    print_vec(bucketsort(unsorted, min, max));
}

