# Parallel Password Cracker

A multi-threaded implementation of a password cracker that can find the password associated with an MD5 hash.

### Prerequisites

A C++ compiler that supports c++11

### Installing

Run the following command at the root problem_3 directory to generate a release build.
```
make
```

## Running

The release executable is located at ./brute_forcer  
The command line arguments give two options for arguments, the first being an MD5 hash, and the second thread count.\
For example:
```
./brute_forcer 2ba56b8acc7fb4d0657532fb6f75b98a 8
```

## Authors

* **Khalid Akash**
* **Brandon Smith**
* **Suva Shahria**
* **Ryan Morey**

## License

This project is licensed under the MIT License - see the [LICENSE.md](../LICENSE.md) file for details
