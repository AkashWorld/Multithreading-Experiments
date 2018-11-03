# Parallel Minimum Spanning Tree

Benchmarks of a minimum spanning tree algorithm. The implementation is done in the Rust Programming Language. The graph that the algorithm works on is implemented as an adjency list. Due to the limited implementation of a pariority queue in the standard rust library, a custom minimum heap was implemented to allow for quick lookup and heapify operations. 

### Prerequisites

The Rust Programming Language
https://www.rust-lang.org/en-US/install.html

### Installing

Run the following command at the root problem_2 directory
```
cargo build --release
```
Run the executable 
```
./target/release/problem_2
```
Alternatively, to quickly run a debug build, simply run the following command at the root problem_2 directory. **Warning, this can take a very long time to run.**
```
cargo run
```

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
