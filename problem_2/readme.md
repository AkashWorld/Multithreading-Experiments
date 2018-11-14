# Parallel Minimum Spanning Tree

Benchmarks of a minimum spanning tree algorithm (Prims). The implementation is done in the Rust Programming Language. The graph that the algorithm works on is implemented as an adjacency list. Due to the limited implementation of a priority queue in the standard rust library, a custom minimum heap was implemented to allow for quick lookup and heapify operations. The parallelization of the algorithm is done at the inner for-loop (neighbor querying). The computation with the parallel algorithm is not CPU-bound, rather is is contention bound unless the graph size is extremely large. Further explanation, analysis, and restrictions are explained in the [writeup.pdf](./writeup.pdf).

### Prerequisites

The Rust Programming Language
https://www.rust-lang.org/en-US/install.html

### Installing

Run the following command at the root problem_2 directory to generate a release build.
```
cargo build --release
```
To generate an unoptimized build, run the following command at the root problem_2 directory
```
cargo build
```
## Running

The release executable is located at ./target/release/problem_2.  
The command line arguments give two options for arguments, the first being the thread count to generate the graphs with, and the second being the total number of nodes in the graph to generate (**Warning, inappopriately setting this number can cause the benchmarks to end due to memory constraints within the system.**) The graph size will be multiplied by 2 from the starting size of 1000 to the set graph size.
For example:
```
./target/release/problem2 8 48000 #Generate a graph with 8 threads, with a maximum of 48000 nodes
```
If no arguments are supplied, the default value of 4 threads and 32000 nodes in the graph  
For example:
```
./target/release/problem2 #Generate a graph with 4 threads, with a default size of 32000 nodes in the graph  
```
```
./target/release/problem2 8 #Generate a graph with 8 threads, with a default size of 32000 nodes in the graph  
```
To quickly build and run debug builds, run the following command, following any of the above listed arguments.  
```
cargo run 8 48000 #Run an unoptimized debug build with 8 threads and 48000 nodes in the graph
```
Tests on our systems show that 16000 nodes are the limit on 8 GB systems, and 48000 nodes are the limit on 16 GB systems. The number of nodes greatly effect the results, so please consult the writeup to see our results.

## Running the tests

Run the following command at the root problem_2 directory
```
cargo test
```

## Built With

* [Rust](https://www.rust-lang.org/en-US/) - The programming language used


## Authors

* **Khalid Akash**
* **Brandon Smith**
* **Suva Shahria**
* **Ryan Morey**

## License

This project is licensed under the MIT License - see the [LICENSE.md](../LICENSE.md) file for details
