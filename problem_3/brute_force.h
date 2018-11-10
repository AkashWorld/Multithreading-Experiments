// This is start of the header guard.  ADD_H can be any unique name.  By convention, we use the name of the header file.
#ifndef _BRUTE_FORCE_H
#define _BRUTE_FORCE_H

#include <stdlib.h>
#include <stdio.h>
#include <string>
#include <cstring>
#include <chrono>  
#include <iostream>
#include <openssl/md5.h>
#include <sstream>
#include <pthread.h>

using namespace std; 

void gen_strs(const char*,string,const int, const size_t);

void gen_arr(char *);

void md5_to_str(unsigned char*);

void str_to_md5(const char*);

void * thread_worker(void *);

#endif
