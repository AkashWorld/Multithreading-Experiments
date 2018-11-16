# Parallel Sort

A multi-threaded implementation of a several sorting algorithms, consisting of bubblesort, mergesort, and bucketsort.

### Prerequisites

A C, and C++ compiler, such as gcc, g++, clang, and/or clang++.

### Installing

Run the following command at the root problem_3 directory to generate a release build.
```
make
```

## Running

Executables have the name bubblesort and mergesort.
For both of the above, two options are required, the array size to run the algorithms on, and the number of threads to use.
For example:
```
./mergesort 16000 8
./bubblesort 16000 8
./bucketsort
```

## Authors

* **Khalid Akash**
* **Brandon Smith**
* **Suva Shahria**
* **Ryan Morey**

## License

This project is licensed under the MIT License - see the [LICENSE.md](../LICENSE.md) file for details
