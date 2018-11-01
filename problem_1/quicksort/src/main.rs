extern crate rand;
use rand::prelude::*;
fn main() {
    let n = 100;
    
    print!("Generating...");
    let mut v = Vec::new();
    for _i in 0..n{
       v.push(random())
    }
    println!("Done!");

    print!("Sorting...");
    quicksort(&mut v);
    println!("Done!");

    print!("Checking...");
    for i in 1..v.len(){
        assert!(v[i] >= v[i-1]);
    }
    println!("Correct!");
}

fn quicksort(v: &mut Vec<i32>){
    let len = v.len()-1;
    quicksort_helper(v, 0, len);
}

fn quicksort_helper(v: &mut Vec<i32>, start: usize, end: usize){
    if start == end {
        return;
    }
    if start + 1 == end {
        if v[start] > v[end]{
            v.swap(start,end);
        }
        return;
    }
    let mut pivot = select_pivot(v, start, end);
    pivot = partition(v, pivot, start, end);
    if pivot > start {
        quicksort_helper(v, start, pivot-1);
    }
    if pivot < end {
        quicksort_helper(v, pivot+1, end);
    }
}

fn select_pivot(_v: &mut Vec<i32>, start: usize, _end: usize) -> usize{
    // TODO make this a bit smarter (median of 3?)
    start
}

fn partition(v: &mut Vec<i32>, pivot_i: usize, start: usize, end: usize) -> usize{
    let mut pivot = pivot_i;
    v.swap(pivot, start);
    pivot = start;
    for i in start+1..end+1{
        if v[i] < v[pivot] {
            v.swap(pivot+1, i);
            v.swap(pivot, pivot+1);
            pivot += 1;
        }
    }
    pivot
} 

