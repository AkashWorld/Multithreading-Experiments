# Parallel Minimum Spanning Tree

Benchmarks of a minimum spanning tree algorithm (Prims). The implementation is done in the Rust Programming Language. The graph that the algorithm works on is implemented as an adjacency list. Due to the limited implementation of a priority queue in the standard rust library, a custom minimum heap was implemented to allow for quick lookup and heapify operations. The parallelization of the algorithm is done at the inner for-loop (neighbor querying). The computation with the parallel algorithm is not CPU-bound, rather is is contention bound. Further explanation, analysis, and restrictions are explained in the [writeup.pdf](./writeup.pdf).

### Prerequisites

The Rust Programming Language
https://www.rust-lang.org/en-US/install.html

### Installing

Run the following command at the root problem_2 directory **Warning, this can take a very long time to execute!**
```
cargo run
```
By default, the above command generates the test graphs with 4 threads. This can be customized to run more threads than normal with the following command.
```
cargo run 8 #Generate graph with 8 threads
```
**For the purposes of the benchmarks, optimization of level 0 is required. This is further explained in the write up.**

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
